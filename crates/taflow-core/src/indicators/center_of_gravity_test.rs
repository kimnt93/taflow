use super::center_of_gravity::CenterOfGravity;
#[test]
fn lifecycle() {
    let mut s = CenterOfGravity::new(2).unwrap();
    s.append(1.0);
    assert!(s.append(2.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
