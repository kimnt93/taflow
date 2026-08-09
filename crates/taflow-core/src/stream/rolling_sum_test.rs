use super::{rolling_sum::RollingSum, StreamingIndicator};

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..96).map(|i| i as f64 * 0.5).collect();
    let mut scalar = RollingSum::new(7).unwrap();
    let scalar_out: Vec<f64> = input
        .iter()
        .map(|&x| scalar.append(x).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = RollingSum::new(7).unwrap();
    let mut bulk_out = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_out);
    for (a, b) in scalar_out.iter().zip(&bulk_out) {
        assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
}
