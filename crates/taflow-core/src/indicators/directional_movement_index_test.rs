use super::directional_movement_index::DirectionalMovementIndex;

fn deterministic_bars(length: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<_> = (0..length)
        .map(|index| 100.0 + index as f64 * 0.03 + ((index * 37) % 29) as f64 * 0.07)
        .collect();
    let high = close
        .iter()
        .enumerate()
        .map(|(index, &value)| value + 0.5 + (index % 7) as f64 * 0.04)
        .collect();
    let low = close
        .iter()
        .enumerate()
        .map(|(index, &value)| value - 0.4 - (index % 5) as f64 * 0.03)
        .collect();
    (high, low, close)
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = DirectionalMovementIndex::new(14).unwrap();
    for index in 0..50 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_all_chunk_splits_and_continuation_are_bitwise_identical() {
    let (high, low, close) = deterministic_bars(257);
    let mut scalar = DirectionalMovementIndex::new(14).unwrap();
    let expected: Vec<_> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&high, &low), &close)| scalar.append(high, low, close).unwrap_or(f64::NAN))
        .collect();
    let continuation = scalar.append(110.0, 108.0, 109.0).map(f64::to_bits);

    for split in 0..=high.len() {
        let mut state = DirectionalMovementIndex::new(14).unwrap();
        let mut actual = Vec::new();
        state
            .extend_slices_into(&high[..split], &low[..split], &close[..split], &mut actual)
            .unwrap();
        state
            .extend_slices_into(&high[split..], &low[split..], &close[split..], &mut actual)
            .unwrap();
        assert_eq!(bits(&actual), bits(&expected));
        assert_eq!(
            state.append(110.0, 108.0, 109.0).map(f64::to_bits),
            continuation
        );
    }
}

#[test]
fn bulk_rejects_misaligned_input_before_mutation() {
    let mut state = DirectionalMovementIndex::new(14).unwrap();
    let mut output = vec![7.0];
    assert!(state
        .extend_slices_into(&[2.0], &[], &[1.5], &mut output)
        .is_err());
    assert_eq!(output, [7.0]);
    assert_eq!(state.value(), None);
}
