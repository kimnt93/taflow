//! Incremental Modified Hikkake recognition (CDLHIKKAKEMOD).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {}
/// Incremental CDLHIKKAKEMOD state.
/// Persistent Rust state or aligned output type for `CandleHikkakeModified`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleHikkakeModified {
    candles: VecDeque<Candle>,
    index: usize,
    pending: Option<(usize, i32, f64)>,
    near_sum: f64,
    value: Option<i32>,
}
impl Default for CandleHikkakeModified {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleHikkakeModified {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(8),
            index: 0,
            pending: None,
            near_sum: 0.0,
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, _open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle { high, low, close };
        let i = self.index;
        self.index += 1;
        let mut result = 0;
        let mut new_pattern = false;
        // Deque holds bars i-8..=i-1; bar j maps to index 8 - (i - j).
        if i >= 10 && self.candles.len() == 8 {
            let a = self.candles[5]; // bar i-3
            let b = self.candles[6]; // bar i-2
            let c = self.candles[7]; // bar i-1
            if c.high < b.high && c.low > b.low && b.high < a.high && b.low > a.low {
                let near = ca_highlow_scalar(NEAR, self.near_sum, b.high, b.low);
                if high < c.high && low < c.low && b.close <= b.low + near {
                    self.pending = Some((i, 100, c.high));
                    result = 100;
                    new_pattern = true;
                } else if high > c.high && low > c.low && b.close >= b.high - near {
                    self.pending = Some((i, -100, c.low));
                    result = -100;
                    new_pattern = true;
                }
            }
            if !new_pattern {
                if let Some((setup, direction, threshold)) = self.pending {
                    if i <= setup + 3
                        && ((direction > 0 && close > threshold)
                            || (direction < 0 && close < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        self.pending = None;
                    } else if i > setup + 3 {
                        self.pending = None;
                    }
                }
            }
            // Slide the sum exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum += cr_highlow_scalar(self.candles[6].high, self.candles[6].low)
                - cr_highlow_scalar(self.candles[1].high, self.candles[1].low);
        } else if (3..8).contains(&i) {
            // Warm-up: seed the sum exactly like the batch prologue.
            self.near_sum += cr_highlow_scalar(high, low);
        }
        if self.candles.len() == 8 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = (i >= 10).then_some(result);
        self.value
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// The monotonic bar counter, the pending setup and `near_sum` are
    /// *carried* through the steady loop in locals rather than reconstructed,
    /// so this needs no from-empty precondition (`near_sum`'s prologue seeds it
    /// one bar off the steady-state eviction, which a tail replay could not
    /// reproduce). Only the eight-bar candle ring is window bounded, and it is
    /// rebuilt from the slice tail afterwards; a `PROLOGUE`-bar per-bar prefix
    /// guarantees the steady loop's `i-7 .. i` reads land inside this slice.
    /// Bit-identical to calling `append` once per bar.
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
        const PROLOGUE: usize = 8;
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
        let mut near_sum = self.near_sum;
        let mut value = self.value;
        // Write through a pre-sized slice rather than `push`: the length
        // write-back of `push` sits on the critical path of every iteration.
        let base = output.len();
        output.resize(base + len - PROLOGUE, 0);
        let scores = &mut output[base..];
        for (slot, i) in scores.iter_mut().zip(PROLOGUE..len) {
            let bar = index;
            index += 1;
            // `bar >= PROLOGUE` here, so the `(3..8)` warm-up seeding branch of
            // `append` is unreachable: bars 8 and 9 simply do nothing.
            if bar < 10 {
                value = None;
                continue;
            }
            let (first_high, first_low) = (high[i - 3], low[i - 3]);
            let (second_high, second_low, second_close) = (high[i - 2], low[i - 2], close[i - 2]);
            let (third_high, third_low) = (high[i - 1], low[i - 1]);
            let mut result = 0;
            let mut new_pattern = false;
            if third_high < second_high
                && third_low > second_low
                && second_high < first_high
                && second_low > first_low
            {
                let near = ca_highlow_scalar(NEAR, near_sum, second_high, second_low);
                if high[i] < third_high && low[i] < third_low && second_close <= second_low + near {
                    pending = Some((bar, 100, third_high));
                    result = 100;
                    new_pattern = true;
                } else if high[i] > third_high
                    && low[i] > third_low
                    && second_close >= second_high - near
                {
                    pending = Some((bar, -100, third_low));
                    result = -100;
                    new_pattern = true;
                }
            }
            if !new_pattern {
                if let Some((setup, direction, threshold)) = pending {
                    if bar <= setup + 3
                        && ((direction > 0 && close[i] > threshold)
                            || (direction < 0 && close[i] < threshold))
                    {
                        result = direction + direction.signum() * 100;
                        pending = None;
                    } else if bar > setup + 3 {
                        pending = None;
                    }
                }
            }
            // Slide the sum exactly like `append`: `+= cr(i-2) - cr(i-7)`.
            near_sum += cr_highlow_scalar(second_high, second_low)
                - cr_highlow_scalar(high[i - 7], low[i - 7]);
            value = Some(result);
            *slot = result;
        }
        self.index = index;
        self.pending = pending;
        self.near_sum = near_sum;
        self.value = value;
        // Rebuild the window-bounded ring so subsequent appends continue
        // bit-identically.
        self.candles.clear();
        for i in (len - PROLOGUE)..len {
            self.candles.push_back(Candle {
                high: high[i],
                low: low[i],
                close: close[i],
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
        self.near_sum = 0.0;
        self.value = None;
    }
}
