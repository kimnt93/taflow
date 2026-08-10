use super::adaptive_cycle::AdaptiveCycle;
#[test]
fn lifecycle() {
    let mut s = AdaptiveCycle::new().unwrap();
    assert!(s.append(1.0).is_none());
    for value in 2..=60 {
        s.append(value as f64);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
