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
    pub fn reset(&mut self) {
        self.previous_close = None;
        self.first.reset();
        self.second.reset();
        self.third.reset();
        self.value = None;
    }
}
