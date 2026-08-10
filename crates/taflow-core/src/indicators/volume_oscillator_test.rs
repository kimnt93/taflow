use super::volume_oscillator::VolumeOscillator;
#[test]
fn lifecycle() {
    let mut state = VolumeOscillator::new(2, 3).unwrap();
    state.append(1.0);
    state.append(2.0);
    assert!(state.append(3.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
