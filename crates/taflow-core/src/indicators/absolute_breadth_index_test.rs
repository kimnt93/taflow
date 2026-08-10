use super::absolute_breadth_index::AbsoluteBreadthIndex;
#[test]
fn lifecycle() {
    let mut s = AbsoluteBreadthIndex::new().unwrap();
    assert_eq!(s.append(2., 5.), Some(3.));
    s.reset();
    assert!(s.value().is_none());
}
