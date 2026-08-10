use super::rectangle_range::RectangleRange;
#[test]
fn lifecycle() {
    let mut s = RectangleRange::new().unwrap();
    for _ in 0..20 {
        s.append(1., 1.01, 0.99, 1.);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
