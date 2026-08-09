use super::average_directional_index::AverageDirectionalIndex;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let high: Vec<f64> = (0..128)
        .map(|i| 100.0 + i as f64 * 0.2 + (i as f64).sin())
        .collect();
    let low: Vec<f64> = high.iter().map(|x| x - 1.5).collect();
    let close: Vec<f64> = high.iter().map(|x| x - 0.5).collect();
    let mut scalar = AverageDirectionalIndex::new(14).unwrap();
    let scalar_out: Vec<f64> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| scalar.append(h, l, c).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = AverageDirectionalIndex::new(14).unwrap();
    let mut bulk_out = Vec::new();
    bulk.extend_slices_into(&high, &low, &close, &mut bulk_out);
    for (a, b) in scalar_out.iter().zip(&bulk_out) {
        assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
    }
    assert_eq!(
        scalar.value().map(f64::to_bits),
        bulk.value().map(f64::to_bits)
    );
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
