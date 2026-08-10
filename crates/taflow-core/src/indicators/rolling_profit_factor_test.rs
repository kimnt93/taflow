use super::rolling_profit_factor::RollingProfitFactor;
#[test]
fn lifecycle() {
    let mut state = RollingProfitFactor::new(2).unwrap();
    assert!(state.append(1.0).is_none());
    assert_eq!(state.append(-0.5), Some(2.0));
    state.reset();
    assert!(state.value().is_none());
}
