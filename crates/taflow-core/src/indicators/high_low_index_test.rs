use super::high_low_index::HighLowIndex;
#[test]
fn lifecycle() {
    let mut s = HighLowIndex::new(2).unwrap();
    s.append(0., 0., 1., 0.);
    assert!(s.append(0., 0., 0., 1.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
