use super::rolling_granger_causality::RollingGrangerCausality;
#[test]
fn lifecycle() {
    let mut s = RollingGrangerCausality::new(5, 1).unwrap();
    for value in 1..5 {
        assert!(s.append(value as f64, (value * value) as f64).is_none());
    }
    assert!(s.append(5.0, 25.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
