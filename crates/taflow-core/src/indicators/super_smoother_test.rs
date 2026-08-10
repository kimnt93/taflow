use super::super_smoother::SuperSmoother;
#[test]
fn lifecycle() {
    let mut s = SuperSmoother::new(3).unwrap();
    assert!(s.append(1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
