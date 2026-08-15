use super::crossover::Crossover;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = Crossover::new();
    assert_eq!(state.append(1.0, 2.0), 0.0);
    assert_eq!(state.append(3.0, 2.0), 1.0);
    assert_eq!(state.value(), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_chunking_and_continuation_match_scalar_replay() {
    let left = [0.0, 2.0, 2.0, 0.0, 3.0, f64::NAN, 4.0];
    let right = [1.0; 7];
    let mut scalar = Crossover::new();
    let expected: Vec<_> = left
        .iter()
        .zip(right)
        .map(|(&left, right)| scalar.append(left, right))
        .collect();

    let mut bulk = Crossover::new();
    let mut actual = Vec::new();
    bulk.extend_slices_into(&left[..1], &right[..1], &mut actual)
        .unwrap();
    bulk.extend_slices_into(&left[1..], &right[1..], &mut actual)
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(bulk.value(), scalar.value());
    assert_eq!(bulk.append(5.0, 2.0), scalar.append(5.0, 2.0));

    let before = actual.clone();
    let value = bulk.value();
    assert!(bulk
        .extend_slices_into(&left, &right[..right.len() - 1], &mut actual)
        .is_err());
    assert_eq!(actual, before);
    assert_eq!(bulk.value(), value);
}
