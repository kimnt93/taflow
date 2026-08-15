use super::inside_bar::InsideBar;

#[test]
fn scalar_bulk_all_splits_continuation_and_reset_are_bitwise_invariant() {
    let high: Vec<f64> = (0..129)
        .map(|index| 100.0 + (index as f64 * 0.19).sin() + (index % 4) as f64 * 0.1)
        .collect();
    let low: Vec<f64> = high
        .iter()
        .enumerate()
        .map(|(index, high)| high - 1.0 - (index % 5) as f64 * 0.07)
        .collect();
    let mut scalar = InsideBar::new();
    let expected: Vec<f64> = high
        .iter()
        .zip(&low)
        .map(|(&high, &low)| scalar.append(high, low).unwrap_or(f64::NAN))
        .collect();
    let scalar_value = scalar.value();
    let continuation = scalar.append(100.2, 99.4);

    for split in 0..=high.len() {
        let mut bulk = InsideBar::new();
        let mut actual = Vec::new();
        bulk.extend_slices_into(&high[..split], &low[..split], &mut actual)
            .unwrap();
        bulk.extend_slices_into(&high[split..], &low[split..], &mut actual)
            .unwrap();
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(&expected) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        assert_eq!(
            bulk.value().map(f64::to_bits),
            scalar_value.map(f64::to_bits)
        );
        assert_eq!(
            bulk.append(100.2, 99.4).map(f64::to_bits),
            continuation.map(f64::to_bits)
        );
    }

    let mut replay = InsideBar::new();
    let mut output = Vec::new();
    replay.extend_slices_into(&high, &low, &mut output).unwrap();
    replay.reset();
    output.clear();
    replay.extend_slices_into(&high, &low, &mut output).unwrap();
    for (actual, expected) in output.iter().zip(&expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn misaligned_bulk_rejects_before_mutation() {
    let mut state = InsideBar::new();
    let mut output = vec![17.0];
    assert!(state
        .extend_slices_into(&[10.0, 9.0], &[8.0], &mut output)
        .is_err());
    assert_eq!(output, [17.0]);
    assert_eq!(state.value(), None);
}
