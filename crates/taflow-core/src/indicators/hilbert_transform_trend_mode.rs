//! Incremental Hilbert Transform trend/cycle mode (HT_TRENDMODE).

use crate::stream::HilbertTransformDominantCyclePhase;

const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
const PRICE_RING: usize = 50;

/// Incremental HT_TRENDMODE state.
/// Persistent Rust state or aligned output type for `HilbertTransformTrendMode`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct HilbertTransformTrendMode {
    index: usize,
    phase: HilbertTransformDominantCyclePhase,
    /// Fixed ring of the last 50 raw prices; `price_head` is the next write slot.
    prices: [f64; PRICE_RING],
    price_head: usize,
    price_count: usize,
    prev_phase: f64,
    trend1: f64,
    trend2: f64,
    trend3: f64,
    days: i32,
    prev_sine: f64,
    prev_leadsine: f64,
    sine: f64,
    leadsine: f64,
    value: Option<i32>,
}
impl Default for HilbertTransformTrendMode {
    fn default() -> Self {
        Self::new()
    }
}
impl HilbertTransformTrendMode {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new() -> Self {
        Self {
            index: 0,
            phase: HilbertTransformDominantCyclePhase::new(),
            prices: [0.0; PRICE_RING],
            price_head: 0,
            price_count: 0,
            prev_phase: 0.0,
            trend1: 0.0,
            trend2: 0.0,
            trend3: 0.0,
            days: 0,
            prev_sine: 0.0,
            prev_leadsine: 0.0,
            sine: 0.0,
            leadsine: 0.0,
            value: None,
        }
    }
    /// Appends one price and returns 0 (cycle) or 1 (trend) after warmup.
    pub fn append(&mut self, input: f64) -> Option<i32> {
        let today = self.index;
        self.index += 1;
        self.prices[self.price_head] = input;
        self.price_head = (self.price_head + 1) % PRICE_RING;
        if self.price_count < PRICE_RING {
            self.price_count += 1;
        }
        self.phase.append(input);
        if today < 37 {
            return None;
        }
        let smooth_period = self.phase.current_smooth_period();
        let phase = self.phase.current_phase();
        let previous_phase = self.prev_phase;
        self.prev_phase = phase;
        self.prev_sine = self.sine;
        self.prev_leadsine = self.leadsine;
        self.sine = (phase * DEG2RAD).sin();
        self.leadsine = ((phase + 45.0) * DEG2RAD).sin();
        let period = (smooth_period + 0.5) as usize;
        let mut average = 0.0;
        // Newest-to-oldest scan, identical order to the previous
        // `prices.iter().rev().take(period)` accumulation.
        let mut idx = self.price_head;
        for _ in 0..period.min(self.price_count) {
            idx = if idx == 0 { PRICE_RING - 1 } else { idx - 1 };
            average += self.prices[idx];
        }
        if period > 0 {
            average /= period as f64;
        }
        let trendline =
            (4.0 * average + 3.0 * self.trend1 + 2.0 * self.trend2 + self.trend3) / 10.0;
        self.trend3 = self.trend2;
        self.trend2 = self.trend1;
        self.trend1 = average;
        let mut trend = 1;
        if (self.sine > self.leadsine && self.prev_sine <= self.prev_leadsine)
            || (self.sine < self.leadsine && self.prev_sine >= self.prev_leadsine)
        {
            self.days = 0;
            trend = 0;
        }
        self.days += 1;
        if (self.days as f64) < 0.5 * smooth_period {
            trend = 0;
        }
        let delta = phase - previous_phase;
        if smooth_period != 0.0
            && delta > 0.67 * 360.0 / smooth_period
            && delta < 1.5 * 360.0 / smooth_period
        {
            trend = 0;
        }
        let price = self.phase.current_smooth_price();
        if trendline != 0.0 && ((price - trendline) / trendline).abs() >= 0.015 {
            trend = 1;
        }
        self.value = (today >= 63).then_some(trend);
        self.value
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
        self.index = 0;
        self.phase.reset();
        self.prices = [0.0; PRICE_RING];
        self.price_head = 0;
        self.price_count = 0;
        self.prev_phase = 0.0;
        self.trend1 = 0.0;
        self.trend2 = 0.0;
        self.trend3 = 0.0;
        self.days = 0;
        self.prev_sine = 0.0;
        self.prev_leadsine = 0.0;
        self.sine = 0.0;
        self.leadsine = 0.0;
        self.value = None;
    }
}
