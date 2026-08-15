use super::hilbert_transform_phasor::{HilbertTransformPhasor, HilbertTransformPhasorValue};

fn assert_bits_equal(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual.is_nan() && expected.is_nan()) || actual.to_bits() == expected.to_bits(),
            "bit mismatch at {index}: {actual:?} != {expected:?}"
        );
    }
}

fn value_bits(value: Option<HilbertTransformPhasorValue>) -> Option<(u64, u64)> {
    value.map(|value| (value.inphase.to_bits(), value.quadrature.to_bits()))
}

#[test]
fn bulk_all_splits_continuation_and_reset_are_bitwise_scalar_equivalent() {
    let input: Vec<_> = (0..137)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 3.0 + index as f64 * 0.01)
        .collect();
    let mut scalar = HilbertTransformPhasor::new();
    let mut expected_inphase = Vec::new();
    let mut expected_quadrature = Vec::new();
    for &input in &input {
        match scalar.append(input) {
            Some(value) => {
                expected_inphase.push(value.inphase);
                expected_quadrature.push(value.quadrature);
            }
            None => {
                expected_inphase.push(f64::NAN);
                expected_quadrature.push(f64::NAN);
            }
        }
    }
    let scalar_value = value_bits(scalar.value());

    for split in 0..=input.len() {
        let mut state = HilbertTransformPhasor::new();
        let mut inphase = Vec::new();
        let mut quadrature = Vec::new();
        state.extend_slice_into(&input[..split], &mut inphase, &mut quadrature);
        state.extend_slice_into(&input[split..], &mut inphase, &mut quadrature);
        assert_bits_equal(&inphase, &expected_inphase);
        assert_bits_equal(&quadrature, &expected_quadrature);
        assert_eq!(value_bits(state.value()), scalar_value);
        assert_eq!(
            value_bits(state.append(101.25)),
            value_bits(scalar.append(101.25))
        );
        scalar.reset();
        scalar.extend_slice_into(&input, &mut Vec::new(), &mut Vec::new());
    }

    scalar.reset();
    assert_eq!(scalar.value(), None);
    let mut replay_inphase = Vec::new();
    let mut replay_quadrature = Vec::new();
    scalar.extend_slice_into(&input, &mut replay_inphase, &mut replay_quadrature);
    assert_bits_equal(&replay_inphase, &expected_inphase);
    assert_bits_equal(&replay_quadrature, &expected_quadrature);
}
