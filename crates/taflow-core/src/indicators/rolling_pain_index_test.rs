use super::rolling_pain_index::RollingPainIndex;
#[test]
fn lifecycle() {
    let mut s = RollingPainIndex::new(2).unwrap();
    s.append(2.0);
    s.append(1.0);
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
