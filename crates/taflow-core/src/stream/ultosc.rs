//! Incremental Ultimate Oscillator (ULTOSC).

use super::Window;
use crate::error::{TaError, TaResult};

#[derive(Debug, Clone)]
struct FlowWindow {
    bp: Window,
    tr: Window,
    bp_sum: f64,
    tr_sum: f64,
}
impl FlowWindow {
    fn new(period: usize) -> TaResult<Self> {
        Ok(Self {
            bp: Window::new(period)?,
            tr: Window::new(period)?,
            bp_sum: 0.0,
            tr_sum: 0.0,
        })
    }
    fn append(&mut self, bp: f64, tr: f64) {
        if let Some(old) = self.bp.push(bp) {
            self.bp_sum -= old;
        }
        if let Some(old) = self.tr.push(tr) {
            self.tr_sum -= old;
        }
        self.bp_sum += bp;
        self.tr_sum += tr;
    }
    fn ready(&self) -> bool {
        self.bp.is_full()
    }
    fn ratio(&self) -> f64 {
        if self.tr_sum > 0.0 {
            self.bp_sum / self.tr_sum
        } else {
            0.0
        }
    }
    fn reset(&mut self) {
        self.bp.clear();
        self.tr.clear();
        self.bp_sum = 0.0;
        self.tr_sum = 0.0;
    }
}

/// Persistent Ultimate Oscillator with constant work per appended HLC bar.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `UltimateOscillator`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct UltimateOscillator {
    previous_close: Option<f64>,
    first: FlowWindow,
    second: FlowWindow,
    third: FlowWindow,
    value: Option<f64>,
}
impl UltimateOscillator {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> TaResult<Self> {
        if timeperiod1 == 0 || timeperiod2 == 0 || timeperiod3 == 0 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: "0".to_string(),
                reason: "must be >= 1",
            });
        }
        Ok(Self {
            previous_close: None,
            first: FlowWindow::new(timeperiod1)?,
            second: FlowWindow::new(timeperiod2)?,
            third: FlowWindow::new(timeperiod3)?,
            value: None,
        })
    }
    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, high: f64, low: f64, close: f64) -> Option<f64> {
        let Some(previous) = self.previous_close.replace(close) else {
            return None;
        };
        let true_low = low.min(previous);
        let true_high = high.max(previous);
        let bp = close - true_low;
        let tr = true_high - true_low;
        self.first.append(bp, tr);
        self.second.append(bp, tr);
        self.third.append(bp, tr);
        self.value = (self.first.ready() && self.second.ready() && self.third.ready()).then(|| {
            100.0 * (4.0 * self.first.ratio() + 2.0 * self.second.ratio() + self.third.ratio())
                / 7.0
        });
        self.value
    }
    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }
    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.first.reset();
        self.second.reset();
        self.third.reset();
        self.value = None;
    }
}

/// Ultimate Oscillator (ULTOSC)
///
/// ULTOSC = 100 * (4*avg7 + 2*avg14 + avg28) / 7
/// Here `avg_n = sum(BP, n) / sum(TR, n)`.
/// BP (Buying Pressure) = close - min(low, prev_close)
/// TR (True Range) = max(high, prev_close) - min(low, prev_close)
pub fn ultimate_oscillator(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    timeperiod1: usize,
    timeperiod2: usize,
    timeperiod3: usize,
) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    let max_period = timeperiod1.max(timeperiod2).max(timeperiod3);
    if len <= max_period {
        return Err(TaError::InsufficientData {
            need: max_period + 1,
            got: len,
        });
    }

    // Compute buying pressure (BP) and true range (TR).
    let mut bp = vec![0.0; len];
    let mut tr = vec![0.0; len];
    for i in 1..len {
        let true_low = low[i].min(close[i - 1]);
        let true_high = high[i].max(close[i - 1]);
        bp[i] = close[i] - true_low;
        tr[i] = true_high - true_low;
    }

    let mut output = vec![0.0_f64; len];
    output[..max_period].fill(f64::NAN);

    // Initialize sliding sums for the first output position (i = max_period)
    // Window for period p at position i covers bp[(i+1-p)..=i]
    let i0 = max_period;
    let mut sum_bp1: f64 = bp[(i0 + 1 - timeperiod1)..=i0].iter().sum();
    let mut sum_tr1: f64 = tr[(i0 + 1 - timeperiod1)..=i0].iter().sum();
    let mut sum_bp2: f64 = bp[(i0 + 1 - timeperiod2)..=i0].iter().sum();
    let mut sum_tr2: f64 = tr[(i0 + 1 - timeperiod2)..=i0].iter().sum();
    let mut sum_bp3: f64 = bp[(i0 + 1 - timeperiod3)..=i0].iter().sum();
    let mut sum_tr3: f64 = tr[(i0 + 1 - timeperiod3)..=i0].iter().sum();

    let avg1 = if sum_tr1 > 0.0 {
        sum_bp1 / sum_tr1
    } else {
        0.0
    };
    let avg2 = if sum_tr2 > 0.0 {
        sum_bp2 / sum_tr2
    } else {
        0.0
    };
    let avg3 = if sum_tr3 > 0.0 {
        sum_bp3 / sum_tr3
    } else {
        0.0
    };
    output[i0] = 100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0;

    // Slide sums forward: add new element, remove oldest
    for i in (max_period + 1)..len {
        sum_bp1 += bp[i] - bp[i - timeperiod1];
        sum_tr1 += tr[i] - tr[i - timeperiod1];
        sum_bp2 += bp[i] - bp[i - timeperiod2];
        sum_tr2 += tr[i] - tr[i - timeperiod2];
        sum_bp3 += bp[i] - bp[i - timeperiod3];
        sum_tr3 += tr[i] - tr[i - timeperiod3];

        let avg1 = if sum_tr1 > 0.0 {
            sum_bp1 / sum_tr1
        } else {
            0.0
        };
        let avg2 = if sum_tr2 > 0.0 {
            sum_bp2 / sum_tr2
        } else {
            0.0
        };
        let avg3 = if sum_tr3 > 0.0 {
            sum_bp3 / sum_tr3
        } else {
            0.0
        };
        output[i] = 100.0 * (4.0 * avg1 + 2.0 * avg2 + avg3) / 7.0;
    }

    Ok(output)
}
