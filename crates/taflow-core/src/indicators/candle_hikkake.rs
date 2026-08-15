//! Incremental Hikkake pattern recognition (CDLHIKKAKE).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    high: f64,
    low: f64,
}
/// Incremental CDLHIKKAKE state.
/// Persistent Rust state or aligned output type for `CandleHikkake`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleHikkake {
    candles: VecDeque<Candle>,
    index: usize,
    pending: Option<(usize, i32, f64)>,
    value: Option<i32>,
}
impl Default for CandleHikkake {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHikkake {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(2),
            index: 0,
            pending: None,
            value: None,
        }
    }
    /// Appends OHLC data and returns a Hikkake setup/confirmation after warmup.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle { high, low };
        let i = self.index;
        self.index += 1;
        let mut result = 0;
        if self.candles.len() == 2 {
            let first = self.candles[0];
            let second = self.candles[1];
            if second.high < first.high
                && second.low > first.low
                && high < second.high
                && low < second.low
            {
                self.pending = Some((i, 100, second.high));
                result = 100;
            } else if second.high < first.high
                && second.low > first.low
                && high > second.high
                && low > second.low
            {
                self.pending = Some((i, -100, second.low));
                result = -100;
            } else if i >= 5 {
                if let Some((setup, direction, threshold)) = self.pending {
                    if i - setup <= 3
                        && ((direction > 0 && close > threshold)
                            || (direction < 0 && close < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        self.pending = None;
                    } else if i - setup > 3 {
                        self.pending = None;
                    }
                }
            }
        }
        if self.candles.len() == 2 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = (i >= 5).then_some(result);
        self.value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// The monotonic bar counter and the pending setup are *carried* through
    /// the steady loop in locals rather than reconstructed, so this needs no
    /// from-empty precondition: only the two-bar candle ring is window bounded,
    /// and it is rebuilt from the slice tail afterwards. A `PROLOGUE`-bar
    /// per-bar prefix guarantees the ring holds nothing but bars of this slice,
    /// after which the steady loop reads `high[i-1]`/`high[i-2]` straight out
    /// of the inputs. Bit-identical to calling `append` once per bar.
    ///
    /// # Parameters
    ///
    /// * `open`, `high`, `low`, `close` - Equal-length chronological OHLC series.
    /// * `output` - Destination the aligned scores are appended to.
    ///
    /// # Returns
    ///
    /// `Ok(())`, or a validation error when the inputs are not aligned.
    pub fn extend_slices_into(
        &mut self,
        open: &[f64],
        high: &[f64],
        low: &[f64],
        close: &[f64],
        output: &mut Vec<i32>,
    ) -> TaResult<()> {
        /// Bars the ring spans; after this many appends it holds only slice bars.
        const PROLOGUE: usize = 2;
        let len = validate_ohlc(open, high, low, close)?;
        output.reserve(len);
        let prologue = len.min(PROLOGUE);
        for i in 0..prologue {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
        if len <= PROLOGUE {
            return Ok(());
        }
        let mut index = self.index;
        let mut pending = self.pending;
        let mut value = self.value;
        // Write through a pre-sized slice rather than `push`: the length
        // write-back of `push` sits on the critical path of every iteration.
        let base = output.len();
        output.resize(base + len - PROLOGUE, 0);
        let scores = &mut output[base..];
        for (slot, i) in scores.iter_mut().zip(PROLOGUE..len) {
            let bar = index;
            index += 1;
            let (first_high, first_low) = (high[i - 2], low[i - 2]);
            let (second_high, second_low) = (high[i - 1], low[i - 1]);
            let (this_high, this_low) = (high[i], low[i]);
            let inside = second_high < first_high && second_low > first_low;
            let mut result = 0;
            if inside && this_high < second_high && this_low < second_low {
                pending = Some((bar, 100, second_high));
                result = 100;
            } else if inside && this_high > second_high && this_low > second_low {
                pending = Some((bar, -100, second_low));
                result = -100;
            } else if bar >= 5 {
                if let Some((setup, direction, threshold)) = pending {
                    if bar - setup <= 3
                        && ((direction > 0 && close[i] > threshold)
                            || (direction < 0 && close[i] < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        pending = None;
                    } else if bar - setup > 3 {
                        pending = None;
                    }
                }
            }
            value = (bar >= 5).then_some(result);
            *slot = value.unwrap_or(0);
        }
        self.index = index;
        self.pending = pending;
        self.value = value;
        // Rebuild the window-bounded ring so subsequent appends continue
        // bit-identically.
        self.candles.clear();
        for i in (len - PROLOGUE)..len {
            self.candles.push_back(Candle {
                high: high[i],
                low: low[i],
            });
        }
        Ok(())
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.candles.clear();
        self.index = 0;
        self.pending = None;
        self.value = None;
    }
}
