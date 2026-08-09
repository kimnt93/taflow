//! Arithmetic shared by the Accumulation/Distribution line and oscillator.

#[inline]
pub(crate) fn money_flow_volume(high: f64, low: f64, close: f64, volume: f64) -> f64 {
    let range = high - low;
    if range > 0.0 {
        ((close - low) - (high - close)) / range * volume
    } else {
        0.0
    }
}
