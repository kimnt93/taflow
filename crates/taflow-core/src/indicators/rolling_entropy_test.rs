use super::rolling_entropy::RollingEntropy;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = RollingEntropy::new(2).unwrap();
    assert_eq!(state.append(1.0), None);
    assert!(state.append(2.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
