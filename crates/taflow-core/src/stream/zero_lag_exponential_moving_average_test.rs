use super::zero_lag_exponential_moving_average::ZeroLagExponentialMovingAverage;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = ZeroLagExponentialMovingAverage::new(5).unwrap();
    for value in 0..8 {
        state.append(value as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
