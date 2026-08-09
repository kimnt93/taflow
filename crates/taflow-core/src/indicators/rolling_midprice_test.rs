use super::RollingMidprice;

#[test]
fn bulk_and_scalar_are_bitwise_equal() {
    let close: Vec<f64> = (0..128).map(|i| (i as f64 * 0.31).cos()).collect();
    let high: Vec<f64> = close.iter().map(|&value| value + 1.0).collect();
    let low: Vec<f64> = close.iter().map(|&value| value - 1.0).collect();
    let mut scalar = RollingMidprice::new(7).unwrap();
    let expected: Vec<f64> = high
        .iter()
        .zip(low.iter())
        .map(|(&high, &low)| scalar.append(high, low).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = RollingMidprice::new(7).unwrap();
    let mut actual = Vec::new();
    bulk.extend_slices_into(&high, &low, &mut actual).unwrap();
    for (left, right) in expected.iter().zip(actual.iter()) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
