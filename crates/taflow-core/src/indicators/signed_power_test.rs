use super::SignedPower;

#[test]
fn signed_power_replays_after_reset() {
    let mut state = SignedPower::new(2.0);
    let first: Vec<_> = [-2.0, 0.0, 3.0]
        .into_iter()
        .map(|v| state.append(v))
        .collect();
    assert_eq!(first, vec![-4.0, 0.0, 9.0]);
    state.reset();
    assert_eq!(state.value(), None);
}
