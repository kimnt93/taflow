use super::operator_states::ContiguousWindow;
use super::rolling_extrema::{MonotonicMax, MonotonicMin};
use crate::error::TaResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwingValue {
    pub signal: f64,
    pub level: f64,
    pub bars_since: f64,
}

#[derive(Debug, Clone)]
pub struct SwingHighLow {
    high_extrema: MonotonicMax,
    low_extrema: MonotonicMin,
    center_highs: ContiguousWindow,
    center_lows: ContiguousWindow,
    bars_since: Option<usize>,
    value: Option<SwingValue>,
}

impl SwingHighLow {
    pub fn new(length: usize) -> TaResult<Self> {
        super::operator_states::validate_period(length)?;
        let capacity = length.saturating_mul(2).saturating_add(1);
        Ok(Self {
            high_extrema: MonotonicMax::new(capacity)?,
            low_extrema: MonotonicMin::new(capacity)?,
            center_highs: ContiguousWindow::new(length + 1),
            center_lows: ContiguousWindow::new(length + 1),
            bars_since: None,
            value: None,
        })
    }
    pub fn append(&mut self, high: f64, low: f64) -> Option<SwingValue> {
        let window_high = self.high_extrema.append(high);
        let window_low = self.low_extrema.append(low);
        self.center_highs.push(high);
        self.center_lows.push(low);
        let (Some(window_high), Some(window_low)) = (window_high, window_low) else {
            self.value = None;
            return None;
        };
        let center_high = self.center_highs.window()[0];
        let center_low = self.center_lows.window()[0];
        let (signal, level) = match (center_high >= window_high, center_low <= window_low) {
            (true, false) => (1.0, center_high),
            (false, true) => (-1.0, center_low),
            _ => (f64::NAN, f64::NAN),
        };
        self.bars_since = if signal.is_nan() {
            self.bars_since.map(|bars| bars + 1)
        } else {
            Some(0)
        };
        let value = SwingValue {
            signal,
            level,
            bars_since: self.bars_since.map_or(f64::NAN, |bars| bars as f64),
        };
        self.value = Some(value);
        Some(value)
    }
    pub fn value(&self) -> Option<SwingValue> {
        self.value
    }
    pub fn bars_since(&self) -> Option<f64> {
        self.bars_since.map(|bars| bars as f64)
    }
    pub fn reset(&mut self) {
        self.high_extrema.reset();
        self.low_extrema.reset();
        self.center_highs.clear();
        self.center_lows.clear();
        self.bars_since = None;
        self.value = None;
    }
}
