use super::exponentially_weighted_standard_deviation::ExponentiallyWeightedStandardDeviation;

#[test]
fn standard_deviation_is_the_variance_root() {
    let mut state = ExponentiallyWeightedStandardDeviation::new(5).unwrap();
    assert_eq!(state.append(7.0), 0.0);
    let value = state.append(9.0);
    assert!(value.is_finite() && value > 0.0);
    state.reset();
    assert_eq!(state.value(), None);
}
