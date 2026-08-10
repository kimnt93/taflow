use super::rolling_omega_ratio::RollingOmegaRatio;

#[test]
fn scalar_replay_and_reset_are_invariant() {
    let values = [1.0, -1.0, 2.0];
    let mut state = RollingOmegaRatio::new(3, 0.0).unwrap();

    assert_eq!(state.append(values[0]), None);
    assert_eq!(state.append(values[1]), None);
    let expected = state.append(values[2]);
    assert!(expected.is_some());
    assert_eq!(state.value(), expected);

    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(values[0]), None);
    assert_eq!(state.append(values[1]), None);
    assert_eq!(state.append(values[2]), expected);
}
