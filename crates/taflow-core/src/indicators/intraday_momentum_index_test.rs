use super::intraday_momentum_index::IntradayMomentumIndex;

#[test]
fn reset_replay_matches() {
    let open: Vec<f64> = (0..64).map(|index| 100.0 + index as f64).collect();
    let close: Vec<f64> = open.iter().map(|value| value + 1.0).collect();
    let mut state = IntradayMomentumIndex::new(14).unwrap();
    let mut output = Vec::new();
    state.extend_slice_into(&open, &close, &mut output).unwrap();
    let final_value = state.value();
    state.reset();
    let mut replay = Vec::new();
    state.extend_slice_into(&open, &close, &mut replay).unwrap();
    for (left, right) in output.iter().zip(replay) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    assert_eq!(state.value(), final_value);
}
