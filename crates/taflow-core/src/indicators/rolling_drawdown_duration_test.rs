use super::rolling_drawdown_duration::RollingDrawdownDuration;
#[test]
fn lifecycle() {
    let mut s = RollingDrawdownDuration::new().unwrap();
    assert_eq!(s.append(2.0), Some(0.0));
    assert_eq!(s.append(1.0), Some(1.0));
    s.reset();
    assert!(s.value().is_none());
}
