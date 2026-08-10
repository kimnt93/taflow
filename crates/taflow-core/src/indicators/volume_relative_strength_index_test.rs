use super::volume_relative_strength_index::VolumeRelativeStrengthIndex;

#[test]
fn matches_wickra_warmup_and_wilder_smoothing() {
    let mut state = VolumeRelativeStrengthIndex::new(3).unwrap();
    assert_eq!(state.append(10.0), None);
    assert_eq!(state.append(20.0), None);
    assert_eq!(state.append(15.0), None);
    assert_eq!(state.append(25.0), Some(80.0));

    let next = state.append(20.0).unwrap();
    assert!((next - 61.53846153846154).abs() < 1e-12);
    assert_eq!(state.value(), Some(next));

    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(10.0), None);
}

#[test]
fn flat_volume_is_neutral_and_period_is_validated() {
    assert!(VolumeRelativeStrengthIndex::new(0).is_err());
    let mut state = VolumeRelativeStrengthIndex::new(2).unwrap();
    assert_eq!(state.append(10.0), None);
    assert_eq!(state.append(10.0), None);
    assert_eq!(state.append(10.0), Some(50.0));
}
