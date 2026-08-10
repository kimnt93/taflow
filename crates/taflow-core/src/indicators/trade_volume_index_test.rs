use super::trade_volume_index::TradeVolumeIndex;
#[test]
fn lifecycle() {
    let mut s = TradeVolumeIndex::new().unwrap();
    s.append(1.0, 10.0);
    assert_eq!(s.append(2.0, 10.0), Some(10.0));
    s.reset();
    assert!(s.value().is_none());
}
