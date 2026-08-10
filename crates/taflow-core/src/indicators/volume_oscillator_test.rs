use super::volume_oscillator::VolumeOscillator;

#[test]
fn matches_wickra_reference_values_and_reset_replay() {
    let volume = [10.0, 20.0, 30.0, 40.0, 50.0];
    let expected = [None, None, None, Some(40.0), Some(1000.0 / 35.0)];
    let mut state = VolumeOscillator::new(2, 4).unwrap();

    let actual: Vec<_> = volume.iter().map(|&value| state.append(value)).collect();
    assert_eq!(actual, expected);
    assert_eq!(state.value(), expected[4]);

    state.reset();
    assert_eq!(state.value(), None);
    let replay: Vec<_> = volume.iter().map(|&value| state.append(value)).collect();
    assert_eq!(replay, expected);
}

#[test]
fn validates_period_order_and_handles_zero_volume() {
    assert!(VolumeOscillator::new(0, 4).is_err());
    assert!(VolumeOscillator::new(4, 4).is_err());
    assert!(VolumeOscillator::new(5, 4).is_err());

    let mut state = VolumeOscillator::new(2, 4).unwrap();
    assert_eq!(state.append(0.0), None);
    assert_eq!(state.append(0.0), None);
    assert_eq!(state.append(0.0), None);
    assert_eq!(state.append(0.0), Some(0.0));
}
