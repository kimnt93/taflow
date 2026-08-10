use super::adaptive_cycle::AdaptiveCycle;
#[test]
fn lifecycle() {
    let mut s = AdaptiveCycle::new().unwrap();
    assert!(s.append(1.0).is_none());
    assert!(s.append(2.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
