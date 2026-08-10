use super::session_volume_weighted_average_price::SessionVolumeWeightedAveragePrice;
#[test]
fn lifecycle() {
    let mut s = SessionVolumeWeightedAveragePrice::new(0).unwrap();
    assert!(s.append(1.0, 2.0, 0.0, 1.0, 2.0, 0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
