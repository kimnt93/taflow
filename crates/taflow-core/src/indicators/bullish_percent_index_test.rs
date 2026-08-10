use super::bullish_percent_index::BullishPercentIndex;
#[test]
fn lifecycle() {
    let mut s = BullishPercentIndex::new().unwrap();
    assert_eq!(s.append(0., 0., 0., 0., 0.5), Some(50.));
    s.reset();
    assert!(s.value().is_none());
}
