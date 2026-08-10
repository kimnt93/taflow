use super::arms_index::ArmsIndex;
#[test]
fn lifecycle() {
    let mut s = ArmsIndex::new().unwrap();
    s.append(1., 2., 0., 0.);
    assert!(s.append(-1., 1., 0., 0.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
