use super::rolling_average_deviation::RollingAverageDeviation;
use crate::stream::StreamingIndicator;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..96).map(|i| (i as f64 * 0.13).cos()).collect();
    let mut scalar = RollingAverageDeviation::new(9).unwrap();
    let scalar_out: Vec<f64> = input
        .iter()
        .map(|&x| scalar.append(x).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = RollingAverageDeviation::new(9).unwrap();
    let mut bulk_out = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_out);
    for (a, b) in scalar_out.iter().zip(&bulk_out) {
        assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}

#[test]
fn chunks_continuation_and_reset_replay_match_scalar_bits() {
    let input: Vec<f64> = (0..173)
        .map(|i| (i as f64 * 0.097).sin() * 5.0 + i as f64 * 0.003)
        .collect();
    let mut scalar = RollingAverageDeviation::new(14).unwrap();
    let expected: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();

    let mut chunked = RollingAverageDeviation::new(14).unwrap();
    let mut actual = Vec::new();
    for chunk in input.chunks(7) {
        chunked.extend_slice_into(chunk, &mut actual);
    }
    for (&actual, &expected) in actual.iter().zip(&expected) {
        assert!(actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan()));
    }
    assert_eq!(chunked.value(), scalar.value());
    assert_eq!(chunked.append(7.25), scalar.append(7.25));

    chunked.reset();
    actual.clear();
    chunked.extend_slice_into(&input, &mut actual);
    for (&actual, &expected) in actual.iter().zip(&expected) {
        assert!(actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan()));
    }
}
