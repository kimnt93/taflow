use super::exponentially_weighted_covariance::ExponentiallyWeightedCovariance;

#[test]
fn covariance_lifecycle_is_causal() {
    let mut state = ExponentiallyWeightedCovariance::new(4).unwrap();
    assert_eq!(state.append(1.0, 2.0), 0.0);
    assert!(state.append(2.0, 4.0).is_finite());
    state.reset();
    assert_eq!(state.value(), None);
}
