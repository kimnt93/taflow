use super::cup_and_handle::CupAndHandle;
#[test]
fn lifecycle() {
    let mut s = CupAndHandle::new().unwrap();
    for i in 0..20 {
        s.append(1., 2., 0., i as f64);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
