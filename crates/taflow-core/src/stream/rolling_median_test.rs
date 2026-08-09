use super::rolling_median::RollingMedian;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..64).map(|i| ((i * 17) % 23) as f64).collect();
    let mut scalar = RollingMedian::new(5).unwrap();
    let scalar_out: Vec<f64> = input
        .iter()
        .map(|&x| scalar.append(x).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = RollingMedian::new(5).unwrap();
    let mut bulk_out = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_out);
    assert_eq!(scalar_out.len(), bulk_out.len());
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
