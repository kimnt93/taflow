use super::roofing_filter::RoofingFilter;
#[test]
fn lifecycle() {
    let mut s = RoofingFilter::new(3, 5).unwrap();
    assert!(s.append(1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
