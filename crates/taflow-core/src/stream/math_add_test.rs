use super::math_add::MathAdd;

#[test]
fn lifecycle_and_bulk_are_consistent() {
    let left = [1.0, -2.0, 4.5];
    let right = [3.0, 7.0, -0.5];
    let mut state = MathAdd::new().unwrap();
    let scalar: Vec<_> = left
        .iter()
        .zip(right)
        .map(|(&left, right)| state.append(left, right))
        .collect();
    assert_eq!(scalar, [4.0, 5.0, 4.0]);
    assert_eq!(state.value(), Some(4.0));
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slices_into(&left, &right, &mut bulk).unwrap();
    assert_eq!(bulk, scalar);
    assert!(state
        .extend_slices_into(&left, &right[..2], &mut bulk)
        .is_err());
}
