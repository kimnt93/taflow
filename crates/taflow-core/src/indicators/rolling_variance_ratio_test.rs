use super::rolling_variance_ratio::RollingVarianceRatio;
#[test]
fn lifecycle() {
    let mut s = RollingVarianceRatio::new(2, 2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
