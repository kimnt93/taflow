use super::triple_exponential_moving_average::TripleExponentialMovingAverage;
use super::StreamingIndicator;

#[test]
fn scalar_and_bulk_lifecycles_match() {
    let input: Vec<f64> = (0..96)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 4.0 + index as f64 * 0.03)
        .collect();
    let mut scalar = TripleExponentialMovingAverage::new(7).unwrap();
    let mut scalar_output = Vec::with_capacity(input.len());
    for value in &input {
        scalar_output.push(scalar.append(*value).unwrap_or(f64::NAN));
    }

    let mut bulk = TripleExponentialMovingAverage::new(7).unwrap();
    let mut bulk_output = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_output);
    assert_eq!(scalar_output.len(), bulk_output.len());
    for (scalar, bulk) in scalar_output.iter().zip(&bulk_output) {
        assert!((scalar.is_nan() && bulk.is_nan()) || scalar == bulk);
    }
    assert_eq!(scalar.value(), bulk.value());

    bulk.reset();
    assert_eq!(bulk.value(), None);
    let mut replay = Vec::new();
    bulk.extend_slice_into(&input, &mut replay);
    for (replayed, scalar) in replay.iter().zip(&scalar_output) {
        assert!((replayed.is_nan() && scalar.is_nan()) || replayed == scalar);
    }
}

#[test]
fn rejects_periods_below_two() {
    assert!(TripleExponentialMovingAverage::new(0).is_err());
    assert!(TripleExponentialMovingAverage::new(1).is_err());
}
