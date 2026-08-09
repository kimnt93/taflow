use super::money_flow_index::MoneyFlowIndex;

#[test]
fn bulk_matches_scalar_mfi() {
    let high: Vec<f64> = (0..128).map(|index| 100.0 + index as f64).collect();
    let low: Vec<f64> = high.iter().map(|value| value - 1.0).collect();
    let close: Vec<f64> = high.iter().map(|value| value - 0.5).collect();
    let volume: Vec<f64> = (0..128).map(|index| 1000.0 + index as f64).collect();
    let mut scalar = MoneyFlowIndex::new(14).unwrap();
    let expected: Vec<f64> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .zip(&volume)
        .map(|(((h, l), c), v)| scalar.append(*h, *l, *c, *v).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = MoneyFlowIndex::new(14).unwrap();
    let mut actual = Vec::new();
    bulk.extend_slices_into(&high, &low, &close, &volume, &mut actual)
        .unwrap();
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}
