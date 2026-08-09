use super::SmoothedTrendChannel;

#[test]
fn bulk_and_scalar_are_bitwise_equal() {
    let high: Vec<f64> = (0..128).map(|i| 100.0 + i as f64 * 0.2).collect();
    let low: Vec<f64> = high.iter().map(|&value| value - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|&value| value - 0.5).collect();
    let mut scalar = SmoothedTrendChannel::new(10).unwrap();
    let expected: Vec<(f64, f64)> = high
        .iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&high, &low), &close)| {
            scalar
                .append(high, low, close)
                .unwrap_or((f64::NAN, f64::NAN))
        })
        .collect();
    let mut bulk = SmoothedTrendChannel::new(10).unwrap();
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    bulk.extend_slice_into(&high, &low, &close, &mut lower, &mut upper)
        .unwrap();
    for ((expected_lower, expected_upper), (&actual_lower, &actual_upper)) in
        expected.iter().zip(lower.iter().zip(upper.iter()))
    {
        assert_eq!(expected_lower.to_bits(), actual_lower.to_bits());
        assert_eq!(expected_upper.to_bits(), actual_upper.to_bits());
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
