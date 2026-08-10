use super::rolling_standard_error::RollingStandardError;
#[test]
fn lifecycle() {
    let mut s = RollingStandardError::new(3).unwrap();
    s.append(1.0);
    s.append(2.0);
    assert!(s.append(3.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
