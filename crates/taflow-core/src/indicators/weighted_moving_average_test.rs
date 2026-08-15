use super::weighted_moving_average::WeightedMovingAverage;
use super::StreamingIndicator;

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn scalar_bulk_chunks_continuation_and_reset_are_bitwise_invariant() {
    let input: Vec<f64> = (0..257)
        .map(|index| (index as f64 * 0.13).cos() * 7.0 + index as f64 * 0.017)
        .collect();

    for period in [1, 2, 9, 30] {
        let mut scalar = WeightedMovingAverage::new(period).unwrap();
        let expected: Vec<f64> = input
            .iter()
            .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
            .collect();

        let mut bulk = WeightedMovingAverage::new(period).unwrap();
        let mut actual = Vec::new();
        bulk.extend_slice_into(&input, &mut actual);
        assert_bits_equal(&actual, &expected);
        assert_eq!(
            bulk.value().map(f64::to_bits),
            scalar.value().map(f64::to_bits)
        );
        assert_eq!(
            bulk.append(-3.25).map(f64::to_bits),
            scalar.append(-3.25).map(f64::to_bits)
        );

        let mut chunked = WeightedMovingAverage::new(period).unwrap();
        let mut chunked_output = Vec::new();
        chunked.extend_slice_into(&input[..3], &mut chunked_output);
        chunked.extend_slice_into(&input[3..41], &mut chunked_output);
        chunked.extend_slice_into(&input[41..], &mut chunked_output);
        assert_bits_equal(&chunked_output, &expected);

        bulk.reset();
        let mut replay = Vec::new();
        bulk.extend_slice_into(&input, &mut replay);
        assert_bits_equal(&replay, &expected);
        assert_eq!(
            bulk.value().map(f64::to_bits),
            expected.last().map(|v| v.to_bits())
        );
    }
}
