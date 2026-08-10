use super::high_low_index::HighLowIndex;
#[test]
fn lifecycle() {
    let mut s = HighLowIndex::new(2).unwrap();
    s.append(1., 0.);
    assert_eq!(s.append(0., 1.), Some(50.));
    s.reset();
    assert!(s.value().is_none());
}
