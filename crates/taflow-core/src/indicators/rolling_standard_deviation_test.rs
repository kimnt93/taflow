use super::rolling_standard_deviation::RollingStandardDeviation;
use crate::stream::StreamingIndicator;

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn scalar_bulk_all_splits_continuation_and_reset_are_bitwise_invariant() {
    let input: Vec<f64> = (0..129)
        .map(|index| 100.0 + (index as f64 * 0.13).cos() * 0.07 + index as f64 * 0.0003)
        .collect();

    for period in [2, 5, 14, 30] {
        for nbdev in [0.0, 1.0, 2.5, -1.5] {
            let mut scalar = RollingStandardDeviation::new(period, nbdev).unwrap();
            let expected: Vec<f64> = input
                .iter()
                .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
                .collect();

            for split in 0..=input.len() {
                let mut bulk = RollingStandardDeviation::new(period, nbdev).unwrap();
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

            let mut replay = RollingStandardDeviation::new(period, nbdev).unwrap();
            let mut replay_output = Vec::new();
            replay.extend_slice_into(&input, &mut replay_output);
            replay.reset();
            replay_output.clear();
            replay.extend_slice_into(&input, &mut replay_output);
            assert_bits_equal(&replay_output, &expected);
        }
    }
}

#[test]
fn constant_input_uses_talib_zero_threshold() {
    let input = vec![42.0; 64];
    let mut state = RollingStandardDeviation::new(5, 3.0).unwrap();
    let mut output = Vec::new();
    state.extend_slice_into(&input, &mut output);
    assert!(output[..4].iter().all(|value| value.is_nan()));
    assert!(output[4..]
        .iter()
        .all(|&value| value.to_bits() == 0.0f64.to_bits()));
}
