use super::lag::Lag;

#[test]
fn scalar_bulk_chunking_and_reset_are_identical() {
    let input = [2.0, 4.0, 1.0, 8.0, 2.0];
    let expected = [f64::NAN, f64::NAN, 2.0, 4.0, 1.0];
    let mut state = Lag::new(2).unwrap();
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| state.append(input).unwrap_or(f64::NAN))
        .collect();
    assert_eq!(scalar[2..], expected[2..]);
    assert!(scalar[..2].iter().all(|value| value.is_nan()));
    assert_eq!(state.value(), Some(1.0));

    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slice_into(&input[..3], &mut bulk);
    state.extend_slice_into(&input[3..], &mut bulk);
    assert_eq!(bulk[2..], expected[2..]);
    assert!(bulk[..2].iter().all(|value| value.is_nan()));
    assert!(Lag::new(0).is_err());
}

#[test]
fn fresh_bulk_reconstructs_exact_continuation_state() {
    let input = [3.0, -1.0, f64::NAN, 8.0, 5.0, 13.0, -2.0];
    let mut scalar_state = Lag::new(3).unwrap();
    let scalar: Vec<_> = input
        .iter()
        .map(|&value| scalar_state.append(value).unwrap_or(f64::NAN))
        .collect();

    let mut bulk_state = Lag::new(3).unwrap();
    let mut bulk = Vec::new();
    bulk_state.extend_slice_into(&input, &mut bulk);
    assert_eq!(
        bulk.iter().map(|value| value.to_bits()).collect::<Vec<_>>(),
        scalar
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        bulk_state.append(21.0).unwrap().to_bits(),
        scalar_state.append(21.0).unwrap().to_bits()
    );

    for split in 0..=input.len() {
        let mut chunked_state = Lag::new(3).unwrap();
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
