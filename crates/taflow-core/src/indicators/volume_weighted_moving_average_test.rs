use super::volume_weighted_moving_average::VolumeWeightedMovingAverage;

#[test]
fn weighted_window_and_reset_are_consistent() {
    let mut state = VolumeWeightedMovingAverage::new(2).unwrap();
    assert_eq!(state.append(10.0, 1.0), None);
    assert_eq!(state.append(20.0, 3.0), Some(17.5));
    state.reset();
    assert_eq!(state.value(), None);
}
