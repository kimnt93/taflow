use super::breadth_thrust::BreadthThrust;
#[test]
fn lifecycle() {
    let mut s = BreadthThrust::new(2).unwrap();
    s.append(1., 0., 0., 0.);
    assert!(s.append(-1., 0., 0., 0.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
