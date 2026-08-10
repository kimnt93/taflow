use super::volume_by_time_profile::VolumeByTimeProfile;
#[test]
fn lifecycle() {
    let mut s = VolumeByTimeProfile::new(24, 0).unwrap();
    let value = s.append(1., 1., 1., 1., 4., 0).unwrap();
    assert_eq!(value.bins[0], 4.0);
    s.reset();
    assert!(s.value().is_none());
}
