use super::exponentially_weighted_sum::ExponentiallyWeightedSum;

#[test]
fn recurrence_and_reset_are_consistent() {
    let mut state = ExponentiallyWeightedSum::new(3).unwrap();
    assert_eq!(state.append(2.0), 2.0);
    assert_eq!(state.append(4.0), 5.0);
    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(2.0), 2.0);
}
