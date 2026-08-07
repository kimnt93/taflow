//! Incremental Hilbert Transform trend/cycle mode (HT_TRENDMODE).

use crate::stream::HtDcphase;
use std::collections::VecDeque;

const DEG2RAD: f64 = std::f64::consts::PI / 180.0;

/// Incremental HT_TRENDMODE state.
pub struct HtTrendmode {
    index: usize,
    phase: HtDcphase,
    prices: VecDeque<f64>,
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
impl Default for HtTrendmode {
    fn default() -> Self {
        Self::new()
    }
}
impl HtTrendmode {
    pub fn new() -> Self {
        Self {
            index: 0,
            phase: HtDcphase::new(),
            prices: VecDeque::with_capacity(50),
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
        if self.prices.len() == 50 {
            self.prices.pop_front();
        }
        self.prices.push_back(input);
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
        for &price in self.prices.iter().rev().take(period) {
            average += price;
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
    pub fn value(&self) -> Option<i32> {
        self.value
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matches_batch() {
        let input: Vec<f64> = (0..400)
            .map(|i| 100.0 + (i as f64 * 0.11).sin() * 8.0)
            .collect();
        let expected = crate::cycle::hilbert_transform_trend_mode(&input).unwrap();
        let mut state = HtTrendmode::new();
        for (&input, &expected) in input.iter().zip(&expected) {
            match state.append(input) {
                Some(value) => assert_eq!(value, expected),
                None => assert_eq!(expected, 0),
            }
        }
    }
}
