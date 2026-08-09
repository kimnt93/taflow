use super::rolling_correlation::RollingCorrelation;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingCorrelation::new(3).unwrap();
    for (x, y) in [(1.0, 2.0), (4.0, 8.0), (2.0, 4.0), (8.0, 16.0), (3.0, 6.0)] {
        state.append(x, y);
    }
    let expected = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for (x, y) in [(1.0, 2.0), (4.0, 8.0), (2.0, 4.0), (8.0, 16.0), (3.0, 6.0)] {
        state.append(x, y);
    }
    assert_eq!(state.value(), expected);
}
