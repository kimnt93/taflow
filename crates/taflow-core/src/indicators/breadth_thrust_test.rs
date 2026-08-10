use super::breadth_thrust::BreadthThrust;
#[test]
fn lifecycle() {
    let mut s = BreadthThrust::new(2).unwrap();
    s.append(1., 0.);
    assert_eq!(s.append(0., 1.), Some(0.5));
    s.reset();
    assert!(s.value().is_none());
}
