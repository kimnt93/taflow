use super::normalized_average_true_range::NormalizedAverageTrueRange;

#[test]
fn scalar_bulk_reset_and_zero_close_are_bitwise_identical() {
    let close = [10.0_f64, 11.0, 0.0, 12.0, 13.0, 14.0];
    let high = [11.0_f64, 12.0, 1.0, 13.0, 14.0, 15.0];
    let low = [9.0_f64, 10.0, -1.0, 11.0, 12.0, 13.0];
    let mut scalar_state = NormalizedAverageTrueRange::new(2).unwrap();
    let scalar: Vec<_> = (0..close.len())
        .map(|index| {
            scalar_state
                .append(high[index], low[index], close[index])
                .unwrap_or(f64::NAN)
        })
        .collect();
    assert_eq!(scalar[2], 0.0);
    let mut bulk_state = NormalizedAverageTrueRange::new(2).unwrap();
    let mut bulk = Vec::new();
    bulk_state
        .extend_slices_into(&high, &low, &close, &mut bulk)
        .unwrap();
    for (actual, expected) in bulk.iter().zip(&scalar) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(bulk_state.value(), scalar_state.value());
    bulk_state.reset();
    assert_eq!(bulk_state.value(), None);
    assert!(NormalizedAverageTrueRange::new(0).is_err());
}
