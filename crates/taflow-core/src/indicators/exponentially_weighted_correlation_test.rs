use super::exponentially_weighted_correlation::ExponentiallyWeightedCorrelation;

#[test]
fn correlation_is_bounded_for_nonconstant_inputs() {
    let mut state = ExponentiallyWeightedCorrelation::new(4).unwrap();
    for (left, right) in [(1.0, 2.0), (2.0, 4.0), (3.0, 1.0), (4.0, 5.0)] {
        let value = state.append(left, right);
        assert!((-1.0..=1.0).contains(&value));
    }
    state.reset();
    assert_eq!(state.value(), None);
}
