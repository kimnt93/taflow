use super::rolling_min_max_index::RollingMinMaxIndex;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingMinMaxIndex::new(3).unwrap();
    let _ = state.append(1.0);
    let _ = state.append(2.0);
    let _ = state.append(3.0);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.value().is_none());
}

#[test]
fn bulk_matches_scalar_and_preserves_continuation() {
    let input = [3.0, 5.0, 4.0, 5.0, 2.0, 2.0, 8.0, 1.0, 8.0];
    let mut scalar = RollingMinMaxIndex::new(3).unwrap();
    let expected: Vec<_> = input.iter().map(|&value| scalar.append(value)).collect();

    let mut bulk = RollingMinMaxIndex::new(3).unwrap();
    let mut minimum = Vec::new();
    let mut maximum = Vec::new();
    bulk.extend_slices_into(&input[..7], &mut minimum, &mut maximum);
    bulk.extend_slices_into(&input[7..], &mut minimum, &mut maximum);

    assert_eq!(
        minimum,
        expected
            .iter()
            .map(|value| value.minimum as f64)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        maximum,
        expected
            .iter()
            .map(|value| value.maximum as f64)
            .collect::<Vec<_>>()
    );
    assert_eq!(bulk.value(), scalar.value());
}
