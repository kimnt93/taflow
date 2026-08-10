use super::overnight_gap::OvernightGap;
#[test]
fn lifecycle() {
    let mut s = OvernightGap::new(0).unwrap();
    assert!(s.append(1.0, 1.0, 1.0, 2.0, 1.0, 0).is_none());
    assert!(s
        .append(3.0, 3.0, 3.0, 3.0, 1.0, 86_400_000_000_000)
        .is_some());
    s.reset();
    assert!(s.value().is_none());
}
