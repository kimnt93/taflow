use super::rolling_cointegration::RollingCointegration;
#[test]
fn lifecycle() {
    let mut s = RollingCointegration::new(2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 4.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
