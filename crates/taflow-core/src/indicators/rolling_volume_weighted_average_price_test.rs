use super::rolling_volume_weighted_average_price::RollingVolumeWeightedAveragePrice;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = RollingVolumeWeightedAveragePrice::new(2).unwrap();
    assert_eq!(state.append(11.0, 9.0, 10.0, 2.0), None);
    assert_eq!(state.append(12.0, 10.0, 11.0, 2.0), Some(10.5));
    state.reset();
    assert_eq!(state.value(), None);
}
