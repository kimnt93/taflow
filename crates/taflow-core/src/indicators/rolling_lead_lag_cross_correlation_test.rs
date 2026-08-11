use super::rolling_lead_lag_cross_correlation::RollingLeadLagCrossCorrelation;
#[test]
fn lifecycle() {
    let mut s = RollingLeadLagCrossCorrelation::new(2, 1).unwrap();
    s.append(1.0, 2.0);
    s.append(2.0, 3.0);
    assert!(s.append(3.0, 4.0).is_none());
    assert!(s.append(4.0, 5.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
