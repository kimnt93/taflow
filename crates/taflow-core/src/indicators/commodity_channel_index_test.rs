use super::commodity_channel_index::CommodityChannelIndex;

#[test]
fn bulk_and_scalar_cci_match() {
    let high: Vec<f64> = (0..128).map(|index| 100.0 + index as f64).collect();
    let low: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|value| value - 0.8).collect();
    let mut scalar = CommodityChannelIndex::new(14).unwrap();
    let expected: Vec<_> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((h, l), c)| scalar.append(*h, *l, *c))
        .collect();
    let mut bulk = CommodityChannelIndex::new(14).unwrap();
    let actual = bulk.extend_slice(&high, &low, &close).unwrap();
    assert_eq!(actual, expected);
}
