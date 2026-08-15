use super::cross::Cross;

#[test]
fn detects_both_cross_directions_and_resets() {
    let mut state = Cross::new();
    assert_eq!(state.append(0.0, 1.0), 0.0);
    assert_eq!(state.append(2.0, 1.0), 1.0);
    assert_eq!(state.append(0.0, 1.0), 1.0);
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_chunking_and_continuation_match_scalar_replay() {
    let left = [0.0, 2.0, 2.0, 0.0, 3.0, f64::NAN, -1.0];
    let right = [1.0; 7];
    let mut scalar = Cross::new();
    let expected: Vec<_> = left
        .iter()
        .zip(right)
        .map(|(&left, right)| scalar.append(left, right))
        .collect();

    let mut bulk = Cross::new();
    let mut actual = Vec::new();
    bulk.extend_slices_into(&left[..2], &right[..2], &mut actual)
        .unwrap();
    bulk.extend_slices_into(&left[2..], &right[2..], &mut actual)
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(bulk.value(), scalar.value());
    assert_eq!(bulk.append(4.0, 1.0), scalar.append(4.0, 1.0));

    let before = actual.clone();
    let value = bulk.value();
    assert!(bulk
        .extend_slices_into(&left, &right[..right.len() - 1], &mut actual)
        .is_err());
    assert_eq!(actual, before);
    assert_eq!(bulk.value(), value);
}
