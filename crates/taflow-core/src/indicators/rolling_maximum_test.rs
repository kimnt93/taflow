use super::rolling_maximum::RollingMaximum;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingMaximum::new(3).unwrap();
    for value in [4.0, 2.0, 3.0, 1.0, 5.0] {
        state.append(value);
    }
    let expected = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for value in [4.0, 2.0, 3.0, 1.0, 5.0] {
        state.append(value);
    }
    assert_eq!(state.value(), expected);
}
