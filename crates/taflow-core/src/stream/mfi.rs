//! Incremental Money Flow Index (MFI).

use crate::error::{TaError, TaResult};

use super::{invalid_period, Window};

/// Compute the money flow index result for the supplied aligned series.
///
/// # Parameters
///
/// * `high` - Input series or configuration value.
/// * `low` - Input series or configuration value.
/// * `close` - Input series or configuration value.
/// * `volume` - Input series or configuration value.
/// * `timeperiod` - Input series or configuration value.
///
/// # Returns
///
/// An aligned result with TA-Lib-compatible validation and warm-up values.
pub fn money_flow_index(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    volume: &[f64],
    timeperiod: usize,
) -> TaResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
        return Err(crate::TaError::LengthMismatch {
            expected: high.len(),
            got: low.len().min(close.len()).min(volume.len()),
        });
    }
    let mut state = MoneyFlowIndex::new(timeperiod)?;
    let mut output = Vec::new();
    state.extend_slices_into(high, low, close, volume, &mut output)?;
    Ok(output)
}

/// Persistent Money Flow Index with O(1) updates after each HLCV bar.
#[derive(Debug, Clone)]
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
    /// both the new and the evicted signed flow recomputed directly from the
    /// input slices (the recomputation is deterministic, so the evicted value
    /// is bit-identical to what the ring held). Outputs and post-run state
    /// are bit-identical to per-bar [`Self::append`].
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
        let typical = |i: usize| (high[i] + low[i] + close[i]) / 3.0;
        let mut positive_sum = self.positive_sum;
        let mut negative_sum = self.negative_sum;
        let mut last = f64::NAN;
        let mut previous = typical(period);
        let mut old_previous = typical(0);
        for i in (period + 1)..n {
            let typical_price = typical(i);
            let flow = Self::signed_flow(typical_price, previous, volume[i]);
            previous = typical_price;
            // Evicted element: the signed flow generated `period` steps ago.
            let old_typical = typical(i - period);
            let old = Self::signed_flow(old_typical, old_previous, volume[i - period]);
            old_previous = old_typical;
            Self::apply_flow(&mut positive_sum, &mut negative_sum, Some(old), flow);
            last = Self::output(positive_sum, negative_sum);
            output.push(last);
        }
        self.positive_sum = positive_sum;
        self.negative_sum = negative_sum;
        self.previous_typical_price = Some(previous);
        self.value = Some(last);
        // Rebuild the flow ring so subsequent appends continue bit-identically.
        self.flow.clear();
        let mut prev = typical(n - period - 1);
        for i in (n - period)..n {
            let typical_price = typical(i);
            self.flow
                .push(Self::signed_flow(typical_price, prev, volume[i]));
            prev = typical_price;
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_batch_for_chunked_extend_and_replay() {
        let close: Vec<f64> = (0..96)
            .map(|index| 100.0 + index as f64 * 0.11 + (index as f64 * 0.39).sin() * 3.0)
            .collect();
        let high: Vec<f64> = close.iter().map(|value| value + 1.3).collect();
        let low: Vec<f64> = close.iter().map(|value| value - 0.7).collect();
        let volume: Vec<f64> = (0..96).map(|index| 1_000.0 + index as f64 * 17.0).collect();
        let expected = crate::stream::money_flow_index(&high, &low, &close, &volume, 14).unwrap();

        let mut state = MoneyFlowIndex::new(14).unwrap();
        let mut actual = state
            .extend_slice(&high[..41], &low[..41], &close[..41], &volume[..41])
            .unwrap();
        actual.extend(
            state
                .extend_slice(&high[41..], &low[41..], &close[41..], &volume[41..])
                .unwrap(),
        );
        for (actual, expected) in actual.iter().zip(&expected) {
            match actual {
                Some(actual) => assert!((actual - expected).abs() < 1e-10),
                None => assert!(expected.is_nan()),
            }
        }
    }

    fn lcg_series(n: usize, mut state: u64) -> Vec<f64> {
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                90.0 + (state >> 11) as f64 / (1u64 << 53) as f64 * 20.0
            })
            .collect()
    }

    fn assert_same_bits(actual: &[f64], expected: &[f64], label: &str) {
        assert_eq!(actual.len(), expected.len(), "{label}: length");
        for (i, (a, b)) in actual.iter().zip(expected).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "{label}: bar {i}");
        }
    }

    #[allow(clippy::type_complexity)]
    fn hlcv(n: usize, seed: u64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let close = lcg_series(n, seed);
        let spread_hi = lcg_series(n, seed ^ 0xDEAD_BEEF);
        let spread_lo = lcg_series(n, seed ^ 0x1234_5678);
        let volume: Vec<f64> = lcg_series(n, seed ^ 0x0BAD_F00D)
            .iter()
            .map(|v| (v - 88.0) * 1_000.0)
            .collect();
        let high: Vec<f64> = close
            .iter()
            .zip(&spread_hi)
            .map(|(c, s)| c + (s - 89.0).abs() * 0.1)
            .collect();
        let low: Vec<f64> = close
            .iter()
            .zip(&spread_lo)
            .map(|(c, s)| c - (s - 89.0).abs() * 0.1)
            .collect();
        (high, low, close, volume)
    }

    #[test]
    fn mfi_bulk_is_bitwise_identical_to_per_bar_append() {
        let (high, low, close, volume) = hlcv(5_000, 0x5EED_00F1);
        let (th, tl, tc, tv) = hlcv(256, 0x7A11_00F1);
        for period in [2usize, 5, 14, 30, 200] {
            let mut per_bar = MoneyFlowIndex::new(period).unwrap();
            let reference: Vec<f64> = (0..close.len())
                .map(|i| {
                    per_bar
                        .append(high[i], low[i], close[i], volume[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            let tail_reference: Vec<f64> = (0..tc.len())
                .map(|i| {
                    per_bar
                        .append(th[i], tl[i], tc[i], tv[i])
                        .unwrap_or(f64::NAN)
                })
                .collect();

            for chunk in [usize::MAX, 1, 7, 97] {
                let mut state = MoneyFlowIndex::new(period).unwrap();
                let mut out = Vec::new();
                let mut start = 0;
                while start < close.len() {
                    let end = (start + chunk.min(close.len())).min(close.len());
                    state
                        .extend_slices_into(
                            &high[start..end],
                            &low[start..end],
                            &close[start..end],
                            &volume[start..end],
                            &mut out,
                        )
                        .unwrap();
                    start = end;
                }
                let label = format!("MFI p{period} chunk {chunk}");
                assert_same_bits(&out, &reference, &label);
                let tail_out: Vec<f64> = (0..tc.len())
                    .map(|i| state.append(th[i], tl[i], tc[i], tv[i]).unwrap_or(f64::NAN))
                    .collect();
                assert_same_bits(&tail_out, &tail_reference, &format!("{label} tail"));
            }
        }
    }
}
