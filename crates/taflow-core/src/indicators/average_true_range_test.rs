use super::average_true_range::AverageTrueRange;

#[test]
fn scalar_chunked_reset_and_validation_are_bitwise_identical() {
    let close: Vec<_> = (0..257)
        .map(|index| 100.0 + (index as f64 * 0.21).sin())
        .collect();
    let high: Vec<_> = close.iter().map(|value| value + 1.5).collect();
    let low: Vec<_> = close.iter().map(|value| value - 1.0).collect();
    for period in [1_usize, 5, 14, 30] {
        let mut scalar_state = AverageTrueRange::new(period).unwrap();
        let scalar: Vec<_> = (0..close.len())
            .map(|index| {
                scalar_state
                    .append(high[index], low[index], close[index])
                    .unwrap_or(f64::NAN)
            })
            .collect();
        let mut bulk_state = AverageTrueRange::new(period).unwrap();
        let mut bulk = Vec::new();
        bulk_state
            .extend_slices_into(&high[..43], &low[..43], &close[..43], &mut bulk)
            .unwrap();
        bulk_state
            .extend_slices_into(&high[43..], &low[43..], &close[43..], &mut bulk)
            .unwrap();
        for (actual, expected) in bulk.iter().zip(&scalar) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        assert_eq!(bulk_state.value(), scalar_state.value());
        scalar_state.reset();
        assert_eq!(scalar_state.value(), None);
    }
    assert!(AverageTrueRange::new(0).is_err());
    let mut state = AverageTrueRange::new(14).unwrap();
    let mut output = Vec::new();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut output)
        .is_err());
    assert!(output.is_empty());
}
