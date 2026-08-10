use super::rolling_recovery_factor::RollingRecoveryFactor;
#[test]
fn lifecycle() {
    let mut state = RollingRecoveryFactor::new(3).unwrap();
    state.append(1.0);
    state.append(2.0);
    assert!(state.append(1.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
