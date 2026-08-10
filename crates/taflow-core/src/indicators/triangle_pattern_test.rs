use super::triangle_pattern::TrianglePattern;
#[test]
fn lifecycle() {
    let mut s = TrianglePattern::new().unwrap();
    for i in 0..20 {
        s.append(1., 30. - i as f64, i as f64, 2.);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
