use super::klinger_volume_oscillator::KlingerVolumeOscillator;

#[test]
fn bulk_and_reset_replay_match_scalar_state() {
    let high: Vec<f64> = (0..128).map(|index| 100.0 + index as f64 + 1.0).collect();
    let low: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|value| value - 1.0).collect();
    let volume: Vec<f64> = (0..128).map(|index| 1000.0 + index as f64).collect();
    let mut state = KlingerVolumeOscillator::new(5, 8, 3).unwrap();
    let mut oscillator = Vec::new();
    let mut signal = Vec::new();
    state
        .extend_slice_into(&high, &low, &close, &volume, &mut oscillator, &mut signal)
        .unwrap();
    let final_value = state.value();
    state.reset();
    let mut replay_oscillator = Vec::new();
    let mut replay_signal = Vec::new();
    state
        .extend_slice_into(
            &high,
            &low,
            &close,
            &volume,
            &mut replay_oscillator,
            &mut replay_signal,
        )
        .unwrap();
    assert_eq!(oscillator.len(), replay_oscillator.len());
    assert_eq!(signal.len(), replay_signal.len());
    for (left, right) in oscillator.iter().zip(replay_oscillator) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    for (left, right) in signal.iter().zip(replay_signal) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    assert_eq!(state.value(), final_value);
}
