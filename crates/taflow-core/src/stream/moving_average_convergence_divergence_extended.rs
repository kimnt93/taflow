//! Stateful extended Moving Average Convergence/Divergence.
//!
//! MACDEXT delays each fast/slow input stream by the difference between its
//! own lookback and the shared largest lookback, then feeds synchronized
//! differences through the selected signal moving average.

use crate::error::{TaError, TaResult};
use crate::ma_type::MaType;

use super::{
    moving_average_convergence_divergence::MovingAverageConvergenceDivergenceValue,
    moving_average_convergence_divergence_helpers::macd_ema_steady_loop,
    moving_average_dispatcher::MovingAverageDispatcher, StreamingIndicator,
};

/// Incremental MACDEXT with aligned fast/slow seeds.
/// Persistent Rust state or aligned output type for `MovingAverageConvergenceDivergenceExtended`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct MovingAverageConvergenceDivergenceExtended {
    fast: MovingAverageDispatcher,
    slow: MovingAverageDispatcher,
    signal: MovingAverageDispatcher,
    fast_start: usize,
    slow_start: usize,
    index: usize,
    value: Option<MovingAverageConvergenceDivergenceValue>,
}

impl MovingAverageConvergenceDivergenceExtended {
    /// Creates a MACDEXT state with independently selected MA types.
    pub fn new(
        fastperiod: usize,
        fastmatype: MaType,
        slowperiod: usize,
        slowmatype: MaType,
        signalperiod: usize,
        signalmatype: MaType,
    ) -> TaResult<Self> {
        if fastperiod < 2 || slowperiod < 2 || signalperiod == 0 {
            return Err(TaError::InvalidParameter {
                name: "fastperiod/slowperiod/signalperiod",
                value: format!("{fastperiod}/{slowperiod}/{signalperiod}"),
                reason: "fastperiod >= 2, slowperiod >= 2, signalperiod >= 1",
            });
        }
        let (fastperiod, fastmatype, slowperiod, slowmatype) = if fastperiod < slowperiod {
            (fastperiod, fastmatype, slowperiod, slowmatype)
        } else {
            (slowperiod, slowmatype, fastperiod, fastmatype)
        };
        let fast_lookback = fastmatype.lookback(fastperiod);
        let slow_lookback = slowmatype.lookback(slowperiod);
        let largest_lookback = fast_lookback.max(slow_lookback);
        Ok(Self {
            fast: MovingAverageDispatcher::new(fastperiod, fastmatype)?,
            slow: MovingAverageDispatcher::new(slowperiod, slowmatype)?,
            signal: MovingAverageDispatcher::new(signalperiod, signalmatype)?,
            fast_start: largest_lookback - fast_lookback,
            slow_start: largest_lookback - slow_lookback,
            index: 0,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<MovingAverageConvergenceDivergenceValue> {
        let index = self.index;
        self.index += 1;
        let fast = if index >= self.fast_start {
            self.fast.append(input)
        } else {
            None
        };
        let slow = if index >= self.slow_start {
            self.slow.append(input)
        } else {
            None
        };
        self.value = fast.zip(slow).and_then(|(fast, slow)| {
            let macd = fast - slow;
            self.signal
                .append(macd)
                .map(|signal| MovingAverageConvergenceDivergenceValue {
                    macd,
                    signal,
                    histogram: macd - signal,
                })
        });
        self.value
    }

    /// Bulk kernel. When all three moving averages are plain EMAs (the common
    /// TA-Lib default), the warm steady state advances the three EMA
    /// recurrences in one loop with the scalar states held in locals; other
    /// MA types fall back to a per-bar loop with no per-bar allocation.
    /// Bit-identical to per-bar [`Self::append`] in outputs and post-run state.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        macd_out.reserve(inputs.len());
        signal_out.reserve(inputs.len());
        histogram_out.reserve(inputs.len());
        if self.fast.is_sma() && self.slow.is_sma() && self.signal.is_sma() {
            self.extend_slices_fused_sma(inputs, macd_out, signal_out, histogram_out);
            return;
        }
        let fused = self.fast.is_ema() && self.slow.is_ema() && self.signal.is_ema();
        let mut index = 0;
        if fused {
            // Warm-up prologue: per-bar appends until the signal EMA emits.
            while index < inputs.len() && self.value.is_none() {
                Self::push_outputs(
                    self.append(inputs[index]),
                    macd_out,
                    signal_out,
                    histogram_out,
                );
                index += 1;
            }
            if index < inputs.len() {
                let (fast_k, mut fast) = {
                    let state = self.fast.as_ema_mut().expect("EMA fast state");
                    (state.smoothing(), state.current().expect("warm fast EMA"))
                };
                let (slow_k, mut slow) = {
                    let state = self.slow.as_ema_mut().expect("EMA slow state");
                    (state.smoothing(), state.current().expect("warm slow EMA"))
                };
                let (signal_k, mut signal) = {
                    let state = self.signal.as_ema_mut().expect("EMA signal state");
                    (state.smoothing(), state.current().expect("warm signal EMA"))
                };
                let mut ema_state = [fast, slow, signal];
                let last = macd_ema_steady_loop(
                    &inputs[index..],
                    [fast_k, slow_k, signal_k],
                    &mut ema_state,
                    macd_out,
                    signal_out,
                    histogram_out,
                )
                .or(self.value);
                [fast, slow, signal] = ema_state;
                let appended = inputs.len() - index;
                self.fast
                    .as_ema_mut()
                    .expect("EMA fast state")
                    .store_bulk_state(fast, appended);
                self.slow
                    .as_ema_mut()
                    .expect("EMA slow state")
                    .store_bulk_state(slow, appended);
                self.signal
                    .as_ema_mut()
                    .expect("EMA signal state")
                    .store_bulk_state(signal, appended);
                self.index += appended;
                self.value = last;
            }
            return;
        }
        for &input in &inputs[index..] {
            Self::push_outputs(self.append(input), macd_out, signal_out, histogram_out);
        }
    }

    /// Fused bulk kernel for the all-SMA configuration - TA-Lib's `MACDEXT`
    /// default, and therefore the path the benchmark actually exercises.
    ///
    /// The fast and slow legs become sliding sums indexed straight off the
    /// input slice, with the same `sum -= old; sum += input` statement order
    /// `SimpleMovingAverage::append` uses. The signal leg keeps its own ring,
    /// because its input is the MACD line rather than the price slice. Outputs
    /// and post-run state are bit-identical to per-bar [`Self::append`].
    fn extend_slices_fused_sma(
        &mut self,
        inputs: &[f64],
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        let fast_period = self.fast.as_sma_mut().expect("SMA fast state").period();
        let slow_period = self.slow.as_sma_mut().expect("SMA slow state").period();
        // `new` normalizes the periods, so `fast_period <= slow_period`. After
        // `slow_period` per-bar appends both price rings hold nothing but bars
        // of this slice (the fast leg's `fast_start` delay is at most
        // `slow_period - fast_period`, so it too is fully seeded from here),
        // and the evicted element of each is just `inputs[i - period]`.
        let n = inputs.len();
        let prologue = n.min(slow_period);
        for &input in &inputs[..prologue] {
            Self::push_outputs(self.append(input), macd_out, signal_out, histogram_out);
        }
        if n <= slow_period {
            return;
        }
        let mut fast_sum = self.fast.as_sma_mut().expect("SMA fast state").raw_sum();
        let mut slow_sum = self.slow.as_sma_mut().expect("SMA slow state").raw_sum();
        let fast_len = fast_period as f64;
        let slow_len = slow_period as f64;
        let mut last = self.value;
        {
            let signal = self.signal.as_sma_mut().expect("SMA signal state");
            for i in slow_period..n {
                fast_sum -= inputs[i - fast_period];
                fast_sum += inputs[i];
                slow_sum -= inputs[i - slow_period];
                slow_sum += inputs[i];
                let macd = fast_sum / fast_len - slow_sum / slow_len;
                last = signal
                    .append(macd)
                    .map(|signal| MovingAverageConvergenceDivergenceValue {
                        macd,
                        signal,
                        histogram: macd - signal,
                    });
                Self::push_outputs(last, macd_out, signal_out, histogram_out);
            }
        }
        MovingAverageDispatcher::restore_sma_leg(
            self.fast.as_sma_mut().expect("SMA fast state"),
            inputs,
            fast_sum,
        );
        MovingAverageDispatcher::restore_sma_leg(
            self.slow.as_sma_mut().expect("SMA slow state"),
            inputs,
            slow_sum,
        );
        self.index += n - slow_period;
        self.value = last;
    }

    #[inline]
    fn push_outputs(
        value: Option<MovingAverageConvergenceDivergenceValue>,
        macd_out: &mut Vec<f64>,
        signal_out: &mut Vec<f64>,
        histogram_out: &mut Vec<f64>,
    ) {
        match value {
            Some(value) => {
                macd_out.push(value.macd);
                signal_out.push(value.signal);
                histogram_out.push(value.histogram);
            }
            None => {
                macd_out.push(f64::NAN);
                signal_out.push(f64::NAN);
                histogram_out.push(f64::NAN);
            }
        }
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<MovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.signal.reset();
        self.index = 0;
        self.value = None;
    }
}
