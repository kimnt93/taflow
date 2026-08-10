use super::rolling_average_drawdown::RollingAverageDrawdown;
#[test]
fn lifecycle() {
    let mut s = RollingAverageDrawdown::new(3).unwrap();
    s.append(3.0);
    s.append(2.0);
    assert!(s.value().is_none());
    s.append(1.0);
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
