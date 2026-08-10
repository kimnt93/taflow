use super::rolling_conditional_value_at_risk::RollingConditionalValueAtRisk;
#[test]
fn lifecycle() {
    let mut state = RollingConditionalValueAtRisk::new(3, 0.95).unwrap();
    state.append(-1.0);
    state.append(0.0);
    assert!(state.append(1.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
