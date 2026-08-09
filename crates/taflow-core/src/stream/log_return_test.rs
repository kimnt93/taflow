use super::log_return::LogReturn;

#[test]
fn scalar_bulk_chunking_and_reset_are_bitwise_identical() {
    let input = [2.0_f64, 4.0, 1.0, 8.0, 2.0];
    let mut state = LogReturn::new(2).unwrap();
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| state.append(input).unwrap_or(f64::NAN))
        .collect();
    assert!(scalar[..2].iter().all(|value| value.is_nan()));
    assert_eq!(scalar[2].to_bits(), (0.5_f64).ln().to_bits());
    assert_eq!(scalar[3].to_bits(), (2.0_f64).ln().to_bits());
    assert_eq!(scalar[4].to_bits(), (2.0_f64).ln().to_bits());

    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slice_into(&input[..1], &mut bulk);
    state.extend_slice_into(&input[1..], &mut bulk);
    for (actual, expected) in bulk.iter().zip(&scalar) {
        assert!(actual.is_nan() && expected.is_nan() || actual.to_bits() == expected.to_bits());
    }
    assert!(LogReturn::new(0).is_err());
}
