use super::rolling_value_at_risk::RollingValueAtRisk;
#[test]
fn lifecycle() {
    let mut state = RollingValueAtRisk::new(3, 0.95).unwrap();
    assert!(state.append(-1.0).is_none());
    assert!(state.append(0.0).is_none());
    assert!(state.append(1.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
