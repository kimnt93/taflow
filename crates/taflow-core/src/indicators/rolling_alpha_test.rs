use super::rolling_alpha::RollingAlpha;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = RollingAlpha::new(2).unwrap();
    assert_eq!(state.append(1.0, 2.0), None);
    assert!(state.append(2.0, 4.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
