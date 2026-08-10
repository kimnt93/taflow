use super::cumulative_volume_index::CumulativeVolumeIndex;
#[test]
fn lifecycle() {
    let mut s = CumulativeVolumeIndex::new().unwrap();
    assert_eq!(s.append(150., 50.), Some(0.5));
    s.reset();
    assert!(s.value().is_none());
}
