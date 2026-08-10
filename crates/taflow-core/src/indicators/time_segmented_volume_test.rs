use super::time_segmented_volume::TimeSegmentedVolume;
#[test]
fn lifecycle() {
    let mut s = TimeSegmentedVolume::new().unwrap();
    s.append(1.0, 10.0);
    assert!(s.append(2.0, 10.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
