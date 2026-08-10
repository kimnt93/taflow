use super::volume_zone_oscillator::VolumeZoneOscillator;
#[test]
fn lifecycle() {
    let mut state = VolumeZoneOscillator::new(3).unwrap();
    assert!(state.append(1.0, 1.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
