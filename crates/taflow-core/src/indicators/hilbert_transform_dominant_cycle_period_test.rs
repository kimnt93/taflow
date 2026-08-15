use super::hilbert_transform_dominant_cycle_period::HilbertTransformDominantCyclePeriod;

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual.is_nan() && expected.is_nan()) || actual.to_bits() == expected.to_bits(),
            "bit mismatch at {index}: {actual:?} != {expected:?}"
        );
    }
}

#[test]
fn bulk_all_splits_continuation_and_reset_are_bitwise_scalar_equivalent() {
    let input: Vec<_> = (0..137)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 3.0 + index as f64 * 0.01)
        .collect();
    let mut scalar = HilbertTransformDominantCyclePeriod::new();
    let expected: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    let scalar_value = scalar.value();

    for split in 0..=input.len() {
        let mut state = HilbertTransformDominantCyclePeriod::new();
        let mut actual = Vec::new();
        state.extend_slice_into(&input[..split], &mut actual);
        state.extend_slice_into(&input[split..], &mut actual);
        assert_bits_equal(&actual, &expected);
        assert_eq!(
            state.value().map(f64::to_bits),
            scalar_value.map(f64::to_bits)
        );
        assert_eq!(
            state.append(101.25).map(f64::to_bits),
            scalar.append(101.25).map(f64::to_bits)
        );
        scalar.reset();
        scalar.extend_slice_into(&input, &mut Vec::new());
    }

    scalar.reset();
    assert_eq!(scalar.value(), None);
    let mut replay = Vec::new();
    scalar.extend_slice_into(&input, &mut replay);
    assert_bits_equal(&replay, &expected);
}
