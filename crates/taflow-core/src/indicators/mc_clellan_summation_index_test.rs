use super::mc_clellan_summation_index::McClellanSummationIndex;
#[test]
fn lifecycle() {
    let mut s = McClellanSummationIndex::new().unwrap();
    assert_eq!(s.append(3., 1.), Some(0.0));
    s.reset();
    assert!(s.value().is_none());
}
