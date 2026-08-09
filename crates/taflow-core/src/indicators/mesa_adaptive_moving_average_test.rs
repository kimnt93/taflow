use super::mesa_adaptive_moving_average::MesaAdaptiveMovingAverage;
use crate::stream::StreamingIndicator;

#[test]
fn reset_replays_identically() {
    let input: Vec<f64> = (0..200)
        .map(|index| 100.0 + (index as f64 * 0.23).sin() * 9.0)
        .collect();
    let mut state = MesaAdaptiveMovingAverage::new(0.5, 0.05).unwrap();
    let first: Vec<Option<(u64, u64)>> = input
        .iter()
        .map(|&bar| {
            state
                .append(bar)
                .map(|value| (value.mama.to_bits(), value.fama.to_bits()))
        })
        .collect();
    state.reset();
    let second: Vec<Option<(u64, u64)>> = input
        .iter()
        .map(|&bar| {
            state
                .append(bar)
                .map(|value| (value.mama.to_bits(), value.fama.to_bits()))
        })
        .collect();
    assert_eq!(first, second);
}
