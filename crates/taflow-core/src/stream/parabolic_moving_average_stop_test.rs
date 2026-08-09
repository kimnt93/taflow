use super::parabolic_moving_average_stop::ParabolicMovingAverageStop;

#[test]
fn warmup_and_reset_replay_are_deterministic() {
    let high: Vec<f64> = (0..40).map(|index| 100.0 + index as f64 + 2.0).collect();
    let low: Vec<f64> = high.iter().map(|value| value - 4.0).collect();
    let close: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
    let mut state = ParabolicMovingAverageStop::new(10, 3.0).unwrap();
    let mut first = Vec::new();
    let mut trends = Vec::new();
    state
        .extend_slice_into(&high, &low, &close, &mut first, &mut trends)
        .unwrap();
    assert!(first[..9].iter().all(|value| value.is_nan()));
    assert!(state.value().is_some());
    let final_value = state.value();
    state.reset();
    let mut second = Vec::new();
    let mut second_trends = Vec::new();
    state
        .extend_slice_into(&high, &low, &close, &mut second, &mut second_trends)
        .unwrap();
    assert_eq!(first.len(), second.len());
    for (left, right) in first.iter().zip(second) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    assert_eq!(trends, second_trends);
    assert_eq!(state.value(), final_value);
}
