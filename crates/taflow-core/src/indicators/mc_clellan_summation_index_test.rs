use super::mc_clellan_summation_index::McClellanSummationIndex;
#[test]
fn lifecycle() {
    let mut s = McClellanSummationIndex::new().unwrap();
    assert!(s.append(1., 0., 0., 0.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
