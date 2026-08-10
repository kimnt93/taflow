use super::rolling_granger_causality::RollingGrangerCausality;
#[test]
fn lifecycle() {
    let mut s = RollingGrangerCausality::new(3, 1).unwrap();
    s.append(1.0, 2.0);
    s.append(2.0, 3.0);
    assert!(s.append(3.0, 4.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
