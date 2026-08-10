use super::market_facilitation_index::MarketFacilitationIndex;
#[test]
fn lifecycle() {
    let mut s = MarketFacilitationIndex::new().unwrap();
    assert!(s.append(2.0, 1.0, 10.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
