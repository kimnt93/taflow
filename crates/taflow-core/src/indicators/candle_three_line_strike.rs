//! Incremental Three Line Strike recognition (CDL3LINESTRIKE).
use crate::error::TaResult;
use crate::stream::pattern::*;
use std::collections::VecDeque;
#[derive(Clone, Copy)]
struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}
impl Candle {
    fn color(self) -> i32 {
        if self.close >= self.open {
            1
        } else {
            -1
        }
    }
}

/// Incremental CDL3LINESTRIKE state.
/// Persistent Rust state or aligned output type for `CandleThreeLineStrike`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct CandleThreeLineStrike {
    candles: VecDeque<Candle>,
    near_sum: [f64; 2],
    value: Option<i32>,
}
impl Default for CandleThreeLineStrike {
    fn default() -> Self {
        Self::new()
    }
}
impl CandleThreeLineStrike {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            candles: VecDeque::with_capacity(8),
            near_sum: [0.0; 2],
            value: None,
        }
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<i32> {
        let current = Candle {
            open,
            high,
            low,
            close,
        };
        // Deque holds bars i-8..=i-1; bar j maps to index 8 - (i - j).
        let output = if self.candles.len() == 8 {
            let a = self.candles[5]; // bar i-3
            let b = self.candles[6]; // bar i-2
            let c = self.candles[7]; // bar i-1
            let near_a = ca_highlow_scalar(NEAR, self.near_sum[0], a.high, a.low);
            let near_b = ca_highlow_scalar(NEAR, self.near_sum[1], b.high, b.low);
            let color = a.color();
            let same = color == b.color() && color == c.color() && current.color() != color;
            let progressive = if color == 1 {
                b.close > a.close && c.close > b.close
            } else {
                b.close < a.close && c.close < b.close
            };
            let opens = if color == 1 {
                b.open >= a.open.min(a.close)
                    && b.open <= a.close + near_a
                    && c.open >= b.open.min(b.close)
                    && c.open <= b.close + near_b
            } else {
                b.open <= a.open.max(a.close)
                    && b.open >= a.close - near_a
                    && c.open <= b.open.max(b.close)
                    && c.open >= b.close - near_b
            };
            let strike = if color == 1 {
                open >= c.close && close <= a.open
            } else {
                open <= c.close && close >= a.open
            };
            // Slide sums exactly like the batch loop: sum += cr(bar) - cr(bar - 5).
            self.near_sum[0] += cr_highlow_scalar(a.high, a.low)
                - cr_highlow_scalar(self.candles[0].high, self.candles[0].low);
            self.near_sum[1] += cr_highlow_scalar(b.high, b.low)
                - cr_highlow_scalar(self.candles[1].high, self.candles[1].low);
            Some((same && progressive && opens && strike) as i32 * color * 100)
        } else {
            // Warm-up: seed the sums exactly like the batch prologue.
            let i = self.candles.len();
            if i < 5 {
                self.near_sum[0] += cr_highlow_scalar(high, low);
            }
            if (1..6).contains(&i) {
                self.near_sum[1] += cr_highlow_scalar(high, low);
            }
            None
        };
        if self.candles.len() == 8 {
            self.candles.pop_front();
        }
        self.candles.push_back(current);
        self.value = output;
        output
    }
    /// Bulk-append aligned OHLC slices, pushing one score per bar into `output`.
    ///
    /// The two rolling Near sums are carried through the steady loop in locals.
    /// An eight-bar scalar prologue replaces the old duplicate batch kernel and
    /// tail replay while guaranteeing all indexed reads belong to this chunk.
    /// The candle ring is then rebuilt from the final eight slice bars, leaving
    /// exactly the state produced by scalar replay for cold and warmed chunks.
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
        let len = validate_ohlc(open, high, low, close)?;
        const PROLOGUE: usize = 8;
        output.reserve(len);
        let prologue = len.min(PROLOGUE);
        for i in 0..prologue {
            output.push(self.append(open[i], high[i], low[i], close[i]).unwrap_or(0));
        }
        if len <= PROLOGUE {
            return Ok(());
        }

        let [mut near_a_sum, mut near_b_sum] = self.near_sum;
        let base = output.len();
        output.resize(base + len - PROLOGUE, 0);
        for (slot, i) in output[base..].iter_mut().zip(PROLOGUE..len) {
            let (a_open, a_high, a_low, a_close) =
                (open[i - 3], high[i - 3], low[i - 3], close[i - 3]);
            let (b_open, b_high, b_low, b_close) =
                (open[i - 2], high[i - 2], low[i - 2], close[i - 2]);
            let (c_open, c_close) = (open[i - 1], close[i - 1]);
            let color = if a_close >= a_open { 1 } else { -1 };
            let b_color = if b_close >= b_open { 1 } else { -1 };
            let c_color = if c_close >= c_open { 1 } else { -1 };
            let current_color = if close[i] >= open[i] { 1 } else { -1 };
            let mut score = 0;
            if color == b_color && color == c_color && current_color != color {
                let progressive = if color == 1 {
                    b_close > a_close && c_close > b_close
                } else {
                    b_close < a_close && c_close < b_close
                };
                if progressive {
                    let near_a = ca_highlow_scalar(NEAR, near_a_sum, a_high, a_low);
                    let near_b = ca_highlow_scalar(NEAR, near_b_sum, b_high, b_low);
                    let opens = if color == 1 {
                        b_open >= a_open.min(a_close)
                            && b_open <= a_close + near_a
                            && c_open >= b_open.min(b_close)
                            && c_open <= b_close + near_b
                    } else {
                        b_open <= a_open.max(a_close)
                            && b_open >= a_close - near_a
                            && c_open <= b_open.max(b_close)
                            && c_open >= b_close - near_b
                    };
                    if opens {
                        let strike = if color == 1 {
                            open[i] >= c_close && close[i] <= a_open
                        } else {
                            open[i] <= c_close && close[i] >= a_open
                        };
                        score = strike as i32 * color * 100;
                    }
                }
            }
            *slot = score;

            near_a_sum +=
                cr_highlow_scalar(a_high, a_low) - cr_highlow_scalar(high[i - 8], low[i - 8]);
            near_b_sum +=
                cr_highlow_scalar(b_high, b_low) - cr_highlow_scalar(high[i - 7], low[i - 7]);
        }

        self.near_sum = [near_a_sum, near_b_sum];
        self.value = Some(*output.last().expect("non-empty bulk output"));
        self.candles.clear();
        for i in (len - PROLOGUE)..len {
            self.candles.push_back(Candle {
                open: open[i],
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
        self.near_sum = [0.0; 2];
        self.value = None;
    }
}
