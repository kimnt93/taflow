use super::volume_zone_oscillator::VolumeZoneOscillator;

#[test]
fn matches_wickra_reference_values_and_reset_replay() {
    let close = [10.0, 11.0, 10.0, 12.0, 11.0];
    let volume = [100.0, 110.0, 120.0, 130.0, 140.0];
    let expected = [None, None, None, Some(100.0 / 3.0), Some(-500.0 / 13.0)];
    let mut state = VolumeZoneOscillator::new(3).unwrap();

    let actual: Vec<_> = close
        .iter()
        .zip(volume)
        .map(|(&close, volume)| state.append(close, volume))
        .collect();
    assert_eq!(actual, expected);

    state.reset();
    assert_eq!(state.value(), None);
    let replay: Vec<_> = close
        .iter()
        .zip(volume)
        .map(|(&close, volume)| state.append(close, volume))
        .collect();
    assert_eq!(replay, expected);
}

#[test]
fn validates_period_and_handles_zero_volume() {
    assert!(VolumeZoneOscillator::new(0).is_err());

    let mut state = VolumeZoneOscillator::new(2).unwrap();
    assert_eq!(state.append(1.0, 0.0), None);
    assert_eq!(state.append(2.0, 0.0), None);
    assert_eq!(state.append(3.0, 0.0), Some(0.0));
}
