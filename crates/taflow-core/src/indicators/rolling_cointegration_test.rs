use super::rolling_cointegration::RollingCointegration;
#[test]
fn lifecycle() {
    let mut s = RollingCointegration::new(6, 1).unwrap();
    for index in 1..6 {
        assert!(s.append(index as f64, 2.0 * index as f64).is_none());
    }
    assert!(s.append(6.0, 12.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
