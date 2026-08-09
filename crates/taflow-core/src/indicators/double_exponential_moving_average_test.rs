use super::double_exponential_moving_average::DoubleExponentialMovingAverage;
use super::StreamingIndicator;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..96).map(|i| 100.0 + (i as f64 * 0.2).sin()).collect();
    let mut scalar = DoubleExponentialMovingAverage::new(10).unwrap();
    let scalar_out: Vec<f64> = input
        .iter()
        .map(|&x| scalar.append(x).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = DoubleExponentialMovingAverage::new(10).unwrap();
    let mut bulk_out = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_out);
    for (a, b) in scalar_out.iter().zip(&bulk_out) {
        assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
