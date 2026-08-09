use super::rolling_calmar::RollingCalmar;

#[test]
fn rolling_drawdown_ratio_warms_and_resets() {
    let mut state = RollingCalmar::new(3).unwrap();
    assert_eq!(state.append(1.0), None);
    assert_eq!(state.append(2.0), None);
    assert!(state.append(1.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
