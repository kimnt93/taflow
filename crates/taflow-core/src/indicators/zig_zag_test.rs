use super::zig_zag::ZigZag;
#[test]
fn lifecycle() {
    let mut s = ZigZag::new(0.05).unwrap();
    assert!(s.append(10.0, 9.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
