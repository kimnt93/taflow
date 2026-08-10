use super::volume_relative_strength_index::VolumeRelativeStrengthIndex;
#[test]
fn lifecycle() {
    let mut s = VolumeRelativeStrengthIndex::new(3).unwrap();
    assert!(s.append(1.0, 10.0).is_none());
    s.append(2.0, 10.0);
    s.append(3.0, 10.0);
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
