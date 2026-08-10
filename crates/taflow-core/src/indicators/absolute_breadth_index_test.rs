use super::absolute_breadth_index::AbsoluteBreadthIndex;
#[test]
fn lifecycle() {
    let mut s = AbsoluteBreadthIndex::new().unwrap();
    assert_eq!(s.append(-2., 1., 0., 0.), Some(2.));
    s.reset();
    assert!(s.value().is_none());
}
