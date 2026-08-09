use super::cumulative_sum::CumulativeSum;

#[test]
fn scalar_bulk_and_reset_are_bitwise_identical() {
    let input = [2.0_f64, 4.0, 1.0, 8.0, 2.0];
    let mut scalar_state = CumulativeSum::new().unwrap();
    assert!(scalar_state.value().is_none());
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| scalar_state.append(input))
        .collect();
    let final_value = scalar_state.value();

    scalar_state.reset();
    assert!(scalar_state.value().is_none());
    let replay: Vec<_> = input
        .iter()
        .map(|&input| scalar_state.append(input))
        .collect();
    assert_eq!(scalar, replay);

    let mut bulk_state = CumulativeSum::new().unwrap();
    let mut bulk = Vec::new();
    bulk_state.extend_slice_into(&input[..2], &mut bulk);
    bulk_state.extend_slice_into(&input[2..], &mut bulk);
    assert_eq!(bulk, scalar);
    assert_eq!(bulk_state.value(), final_value);
}
