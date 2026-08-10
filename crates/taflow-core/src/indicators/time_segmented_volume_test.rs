use super::time_segmented_volume::TimeSegmentedVolume;
#[test]
fn lifecycle() {
    let mut s = TimeSegmentedVolume::new(2).unwrap();
    s.append(1.0, 10.0);
    assert!(s.append(2.0, 10.0).is_none());
    assert!(s.append(3.0, 10.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
