use super::{exponential_moving_average::ExponentialMovingAverage, StreamingIndicator};

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn scalar_bulk_all_splits_continuation_and_reset_are_bitwise_invariant() {
    let input: Vec<f64> = (0..129)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 0.9 + index as f64 * 0.003)
        .collect();

    for period in [1, 2, 10, 30] {
        let mut scalar = ExponentialMovingAverage::new(period).unwrap();
        let expected: Vec<f64> = input
            .iter()
            .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
            .collect();

        for split in 0..=input.len() {
            let mut bulk = ExponentialMovingAverage::new(period).unwrap();
            let mut actual = Vec::new();
            bulk.extend_slice_into(&input[..split], &mut actual);
            bulk.extend_slice_into(&input[split..], &mut actual);
            assert_bits_equal(&actual, &expected);
            assert_eq!(
                bulk.value().map(f64::to_bits),
                scalar.value().map(f64::to_bits)
            );

            let mut expected_continuation = scalar.clone();
            assert_eq!(
                bulk.append(99.75).map(f64::to_bits),
                expected_continuation.append(99.75).map(f64::to_bits)
            );
        }

        let mut replay = ExponentialMovingAverage::new(period).unwrap();
        let mut replay_output = Vec::new();
        replay.extend_slice_into(&input, &mut replay_output);
        replay.reset();
        replay_output.clear();
        replay.extend_slice_into(&input, &mut replay_output);
        assert_bits_equal(&replay_output, &expected);
    }
}
