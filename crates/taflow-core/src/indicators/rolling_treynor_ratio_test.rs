use super::rolling_treynor_ratio::RollingTreynorRatio;

#[test]
fn scalar_replay_and_reset_are_invariant() {
    let pairs = [(1.0, 0.5), (-1.0, -0.5), (2.0, 1.0)];
    let mut state = RollingTreynorRatio::new(3).unwrap();

    assert_eq!(state.append(pairs[0].0, pairs[0].1), None);
    assert_eq!(state.append(pairs[1].0, pairs[1].1), None);
    let expected = state.append(pairs[2].0, pairs[2].1);
    assert!(expected.is_some());
    assert_eq!(state.value(), expected);

    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(pairs[0].0, pairs[0].1), None);
    assert_eq!(state.append(pairs[1].0, pairs[1].1), None);
    assert_eq!(state.append(pairs[2].0, pairs[2].1), expected);
}
