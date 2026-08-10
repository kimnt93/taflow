use super::cumulative_volume_index::CumulativeVolumeIndex;
#[test]
fn lifecycle() {
    let mut s = CumulativeVolumeIndex::new().unwrap();
    assert_eq!(s.append(1., 3., 0., 0.), Some(3.));
    s.reset();
    assert!(s.value().is_none());
}
