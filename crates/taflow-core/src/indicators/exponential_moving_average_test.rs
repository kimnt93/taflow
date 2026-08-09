use super::{exponential_moving_average::ExponentialMovingAverage, StreamingIndicator};

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..96).map(|i| 100.0 + (i as f64 * 0.17).sin()).collect();
    let mut scalar = ExponentialMovingAverage::new(10).unwrap();
    let scalar_out: Vec<f64> = input
        .iter()
        .map(|&x| scalar.append(x).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = ExponentialMovingAverage::new(10).unwrap();
    let bulk_out: Vec<f64> = bulk
        .extend_slice(&input)
        .into_iter()
        .map(|x| x.unwrap_or(f64::NAN))
        .collect();
    for (a, b) in scalar_out.iter().zip(&bulk_out) {
        assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
