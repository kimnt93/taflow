use super::zig_zag::ZigZag;
#[test]
fn lifecycle() {
    let mut s = ZigZag::new(0.05).unwrap();
    assert!(s.append(10.0, 9.9).is_none());
    assert!(s.append(12.0, 11.9).is_none());
    let swing = s.append(11.0, 10.0).expect("drop confirms the high");
    assert_eq!(swing.swing, 12.0);
    assert_eq!(swing.direction, 1.0);
    s.reset();
    assert!(s.value().is_none());
}
