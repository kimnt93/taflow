use super::rising::Rising;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = Rising::new(2).unwrap();
    assert_eq!(state.append(1.0), None);
    assert_eq!(state.append(2.0), None);
    assert_eq!(state.append(3.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn scalar_bulk_chunking_and_continuation_are_bit_identical() {
    let input = [3.0, -1.0, 2.0, 8.0, 5.0, 13.0, -2.0];
    let mut scalar_state = Rising::new(3).unwrap();
    let scalar: Vec<_> = input
        .iter()
        .map(|&value| scalar_state.append(value).unwrap_or(f64::NAN))
        .collect();

    let mut bulk_state = Rising::new(3).unwrap();
    let mut bulk = Vec::new();
    bulk_state.extend_slice_into(&input, &mut bulk);
    assert_eq!(
        bulk.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
        scalar
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(bulk_state.append(21.0), scalar_state.append(21.0));

    for split in 0..=input.len() {
        let mut chunked_state = Rising::new(3).unwrap();
        let mut chunked = Vec::new();
        chunked_state.extend_slice_into(&input[..split], &mut chunked);
        chunked_state.extend_slice_into(&input[split..], &mut chunked);
        assert_eq!(
            chunked
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            scalar
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
    }
}
