use super::true_range::TrueRange;

#[test]
fn scalar_bulk_reset_and_validation_are_bitwise_identical() {
    let high = [10.0_f64, 13.0, 12.0, 15.0];
    let low = [8.0_f64, 11.0, 9.0, 12.0];
    let close = [9.0_f64, 12.0, 10.0, 14.0];
    let mut scalar_state = TrueRange::new().unwrap();
    let scalar: Vec<_> = (0..high.len())
        .map(|index| {
            scalar_state
                .append(high[index], low[index], close[index])
                .unwrap_or(f64::NAN)
        })
        .collect();
    assert!(scalar[0].is_nan());
    assert_eq!(&scalar[1..], &[4.0, 3.0, 5.0]);
    scalar_state.reset();
    assert_eq!(scalar_state.value(), None);
    let mut bulk = Vec::new();
    scalar_state
        .extend_slices_into(&high, &low, &close, &mut bulk)
        .unwrap();
    for (actual, expected) in bulk.iter().zip(&scalar) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    let mut chunked_state = TrueRange::new().unwrap();
    let mut chunked = Vec::new();
    chunked_state
        .extend_slices_into(&high[..2], &low[..2], &close[..2], &mut chunked)
        .unwrap();
    chunked_state
        .extend_slices_into(&high[2..], &low[2..], &close[2..], &mut chunked)
        .unwrap();
    assert_eq!(
        chunked
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        bulk.iter().map(|value| value.to_bits()).collect::<Vec<_>>()
    );
    assert_eq!(chunked_state.value(), scalar_state.value());
    let before = bulk.len();
    assert!(scalar_state
        .extend_slices_into(&high, &low[..3], &close, &mut bulk)
        .is_err());
    assert_eq!(bulk.len(), before);
}
