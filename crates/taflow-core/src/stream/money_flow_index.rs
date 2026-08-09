//! Incremental Money Flow Index (MFI).

use crate::error::{TaError, TaResult};

use super::{invalid_period, Window};

/// Persistent Rust state or aligned output type for `MoneyFlowIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MoneyFlowIndex {
    period: usize,
    previous_typical_price: Option<f64>,
    /// One ring of SIGNED money flows (M4 dedup): positive bars store `+mf`,
    /// negative bars store `-mf`, flat bars store `0.0`. A sign test on the
    /// evicted element maintains both directional sums with the exact
    /// arithmetic the two separate rings used before (`x - m ≡ x + (-m)`,
    /// and `x ± 0.0` is a bitwise no-op because neither sum can be `-0.0`).
    flow: Window,
    positive_sum: f64,
    negative_sum: f64,
    value: Option<f64>,
    /// Reusable signed-money-flow scratch for [`Self::extend_slices_into`].
    /// Held on the state so repeated bulk calls allocate at most once; never
    /// touched by [`Self::append`].
    flow_scratch: Vec<f64>,
}

impl MoneyFlowIndex {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(period: usize) -> TaResult<Self> {
        if period < 2 {
            return Err(invalid_period("timeperiod", period, 2));
        }
        Ok(Self {
            period,
            previous_typical_price: None,
            flow: Window::new(period)?,
            positive_sum: 0.0,
            negative_sum: 0.0,
            value: None,
            flow_scratch: Vec::new(),
        })
    }

    /// The signed money flow for the bar transition `previous -> typical`.
    #[inline]
    fn signed_flow(typical_price: f64, previous: f64, volume: f64) -> f64 {
        let money_flow = typical_price * volume;
        if typical_price > previous {
            money_flow
        } else if typical_price < previous {
            -money_flow
        } else {
            0.0
        }
    }

    /// Applies one signed flow to the directional sums (evict then add).
    #[inline]
    fn apply_flow(positive_sum: &mut f64, negative_sum: &mut f64, evicted: Option<f64>, flow: f64) {
        if let Some(old) = evicted {
            if old > 0.0 {
                *positive_sum -= old;
            } else if old < 0.0 {
                *negative_sum += old;
            }
        }
        if flow > 0.0 {
            *positive_sum += flow;
        } else if flow < 0.0 {
            *negative_sum -= flow;
        }
    }

    /// Branchless form of [`Self::apply_flow`] for the bulk steady loop.
    ///
    /// The sign of a money flow is essentially a coin flip bar to bar, so the
    /// four data-dependent branches above mispredict roughly half the time.
    /// Splitting each signed flow into the directional component it would have
    /// contributed - and `0.0` otherwise - turns them into selects. This is
    /// bit-exact, not an approximation: `x - 0.0` and `x + 0.0` are identity on
    /// every f64 except `-0.0`, and neither sum can ever be `-0.0` (both start
    /// at `+0.0`, and IEEE round-to-nearest gives `a - a == +0.0`).
    #[inline]
    fn apply_flow_branchless(
        positive_sum: &mut f64,
        negative_sum: &mut f64,
        evicted: f64,
        flow: f64,
    ) {
        let positive_evicted = if evicted > 0.0 { evicted } else { 0.0 };
        let negative_evicted = if evicted < 0.0 { evicted } else { 0.0 };
        let positive_flow = if flow > 0.0 { flow } else { 0.0 };
        let negative_flow = if flow < 0.0 { flow } else { 0.0 };
        *positive_sum = (*positive_sum - positive_evicted) + positive_flow;
        *negative_sum = (*negative_sum + negative_evicted) - negative_flow;
    }

    #[inline]
    fn output(positive_sum: f64, negative_sum: f64) -> f64 {
        if negative_sum > 0.0 {
            100.0 - 100.0 / (1.0 + positive_sum / negative_sum)
        } else {
            100.0
        }
    }

    /// Appends one HLCV bar and returns MFI after `timeperiod` price changes.
    pub fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let typical_price = (high + low + close) / 3.0;
        let Some(previous) = self.previous_typical_price.replace(typical_price) else {
            return None;
        };

        let flow = Self::signed_flow(typical_price, previous, volume);
        let evicted = self.flow.push(flow);
        Self::apply_flow(
            &mut self.positive_sum,
            &mut self.negative_sum,
            evicted,
            flow,
        );

        self.value = self
            .flow
            .is_full()
            .then(|| Self::output(self.positive_sum, self.negative_sum));
        self.value
    }

    /// Bulk kernel: O(1) add/evict recurrence on the directional sums with
    /// both the new and the evicted signed flow derived directly from the
    /// input slices (the derivation is deterministic, so the evicted value is
    /// bit-identical to what the ring held). Outputs and post-run state are
    /// bit-identical to per-bar [`Self::append`].
    ///
    /// Two things keep the steady loop off the divider and off the branch
    /// predictor. The signed money flows are produced up front in one flat,
    /// branch-free, autovectorizable pass - each element is the same
    /// `(h + l + c) / 3.0` typical price and the same three-way sign test that
    /// `append` evaluates, so it is bit-identical element for element - and the
    /// loop then only slides two sums over that array, splitting each flow with
    /// [`Self::apply_flow_branchless`] rather than four coin-flip branches. The
    /// flow logic was measured at roughly half this kernel's cost before it was
    /// hoisted.
    pub fn extend_slices_into(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
        output: &mut Vec<f64>,
    ) -> TaResult<()> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()).min(volume.len()),
            });
        }
        let period = self.period;
        let n = high.len();
        output.reserve(n);
        // Warm-up prologue. After `period + 1` appends the flow ring holds
        // exactly the flows of steps 1..=period of this slice (step 0 only
        // contributes when a previous typical price already existed, and the
        // ring keeps just the trailing `period` flows either way).
        let prologue = n.min(period + 1);
        for i in 0..prologue {
            output.push(
                self.append(high[i], low[i], close[i], volume[i])
                    .unwrap_or(f64::NAN),
            );
        }
        if n <= period + 1 {
            return Ok(());
        }
        // Flat prepass: `flows[i]` is the signed money flow of the transition
        // `i-1 -> i`, element for element what `Self::signed_flow` would return.
        // Element 0 has no predecessor and is never read.
        let mut flows = std::mem::take(&mut self.flow_scratch);
        flows.clear();
        flows.reserve(n);
        flows.push(0.0);
        flows.extend((1..n).map(|i| {
            let typical_price = (high[i] + low[i] + close[i]) / 3.0;
            let previous = (high[i - 1] + low[i - 1] + close[i - 1]) / 3.0;
            Self::signed_flow(typical_price, previous, volume[i])
        }));

        let mut positive_sum = self.positive_sum;
        let mut negative_sum = self.negative_sum;
        let mut last = f64::NAN;
        // Write through a pre-sized slice: `push`'s length write-back would sit
        // on the critical path of every iteration.
        let base = output.len();
        output.resize(base + n - period - 1, f64::NAN);
        let results = &mut output[base..];
        for (slot, i) in results.iter_mut().zip((period + 1)..n) {
            // Evicted element: the signed flow generated `period` steps ago.
            Self::apply_flow_branchless(
                &mut positive_sum,
                &mut negative_sum,
                flows[i - period],
                flows[i],
            );
            last = Self::output(positive_sum, negative_sum);
            *slot = last;
        }
        self.positive_sum = positive_sum;
        self.negative_sum = negative_sum;
        self.previous_typical_price = Some((high[n - 1] + low[n - 1] + close[n - 1]) / 3.0);
        self.value = Some(last);
        // Rebuild the flow ring so subsequent appends continue bit-identically.
        self.flow.clear();
        for &flow in &flows[n - period..] {
            self.flow.push(flow);
        }
        self.flow_scratch = flows;
        Ok(())
    }

    /// Computes or updates `extend_slice` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn extend_slice(
        &mut self,
        high: &[f64],
        low: &[f64],
        close: &[f64],
        volume: &[f64],
    ) -> TaResult<Vec<Option<f64>>> {
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(TaError::LengthMismatch {
                expected: high.len(),
                got: low.len().min(close.len()).min(volume.len()),
            });
        }
        Ok(high
            .iter()
            .zip(low)
            .zip(close)
            .zip(volume)
            .map(|(((&high, &low), &close), &volume)| self.append(high, low, close, volume))
            .collect())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Computes or updates `reset` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn reset(&mut self) {
        self.previous_typical_price = None;
        self.flow.clear();
        self.positive_sum = 0.0;
        self.negative_sum = 0.0;
        self.value = None;
    }
}
