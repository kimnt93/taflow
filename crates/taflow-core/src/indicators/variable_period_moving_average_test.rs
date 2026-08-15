use super::variable_period_moving_average::VariablePeriodMovingAverage;
use crate::ma_type::MaType;

fn inputs(length: usize) -> Vec<f64> {
    (0..length)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 8.0 + index as f64 * 0.01)
        .collect()
}

fn periods(length: usize) -> Vec<f64> {
    let requested = [1.9, 3.8, 7.2, 11.9, 50.0];
    (0..length)
        .map(|index| requested[index % requested.len()])
        .collect()
}

#[test]
fn scalar_bulk_chunking_and_reset_are_bitwise_identical() {
    let values = inputs(700);
    let requested = periods(values.len());

    for code in 0..=8 {
        let average_type = MaType::try_from(code).unwrap();
        let mut scalar = VariablePeriodMovingAverage::new(2, 12, average_type).unwrap();
        let expected: Vec<f64> = values
            .iter()
            .zip(&requested)
            .map(|(&value, &period)| scalar.append(value, period).unwrap_or(f64::NAN))
            .collect();

        let mut chunked = VariablePeriodMovingAverage::new(2, 12, average_type).unwrap();
        let mut actual = Vec::new();
        chunked
            .extend_slices_into(&values[..173], &requested[..173], &mut actual)
            .unwrap();
        chunked
            .extend_slices_into(&values[173..], &requested[173..], &mut actual)
            .unwrap();
        assert_eq!(
            actual
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            chunked.value().map(f64::to_bits),
            scalar.value().map(f64::to_bits)
        );

        chunked.reset();
        assert!(chunked.value().is_none());
        let mut replay = Vec::new();
        chunked
            .extend_slices_into(&values, &requested, &mut replay)
            .unwrap();
        assert_eq!(
            replay
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn every_two_chunk_split_and_scalar_continuation_are_bitwise_identical() {
    let values = inputs(96);
    let requested = periods(values.len());

    for code in 0..=8 {
        let average_type = MaType::try_from(code).unwrap();
        let mut scalar = VariablePeriodMovingAverage::new(2, 12, average_type).unwrap();
        let expected: Vec<_> = values
            .iter()
            .zip(&requested)
            .map(|(&value, &period)| scalar.append(value, period).unwrap_or(f64::NAN))
            .collect();
        let expected_continuation = scalar.append(117.25, 7.9);

        for split in 0..=values.len() {
            let mut chunked = VariablePeriodMovingAverage::new(2, 12, average_type).unwrap();
            let mut actual = Vec::new();
            chunked
                .extend_slices_into(&values[..split], &requested[..split], &mut actual)
                .unwrap();
            chunked
                .extend_slices_into(&values[split..], &requested[split..], &mut actual)
                .unwrap();
            assert_eq!(
                actual
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                expected
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                "average type {code}, split {split}"
            );

            let continuation = chunked.append(117.25, 7.9);
            assert_eq!(
                continuation.map(f64::to_bits),
                expected_continuation.map(f64::to_bits),
                "average type {code}, split {split} continuation"
            );
        }
    }
}

#[test]
fn misaligned_bulk_input_is_rejected_before_state_mutation() {
    let mut state = VariablePeriodMovingAverage::new(2, 30, MaType::SimpleMovingAverage).unwrap();
    let mut output = Vec::new();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[2.0], &mut output)
        .is_err());
    assert!(output.is_empty());
    assert!(state.value().is_none());

    let values = inputs(64);
    let requested = periods(values.len());
    state
        .extend_slices_into(&values, &requested, &mut output)
        .unwrap();
    let mut fresh = VariablePeriodMovingAverage::new(2, 30, MaType::SimpleMovingAverage).unwrap();
    let mut expected = Vec::new();
    fresh
        .extend_slices_into(&values, &requested, &mut expected)
        .unwrap();
    assert_eq!(
        output
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
}
