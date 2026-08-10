use super::wedge_pattern::WedgePattern;
#[test]
fn lifecycle() {
    let mut s = WedgePattern::new().unwrap();
    for i in 0..20 {
        s.append(1., 3. + i as f64, 1. + i as f64, 2.);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
