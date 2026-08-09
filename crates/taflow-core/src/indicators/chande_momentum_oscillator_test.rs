use super::ChandeMomentumOscillator;

#[test]
fn bulk_and_scalar_are_bitwise_equal() {
    let input: Vec<f64> = (0..128).map(|i| (i as f64 * 0.17).sin()).collect();
    let mut scalar = ChandeMomentumOscillator::new(14).unwrap();
    let expected: Vec<f64> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = ChandeMomentumOscillator::new(14).unwrap();
    let mut actual = Vec::new();
    bulk.extend_slice_into(&input, &mut actual);
    for (left, right) in expected.iter().zip(actual.iter()) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    bulk.reset();
    assert_eq!(bulk.value(), None);
    assert!(ChandeMomentumOscillator::new(1).is_err());
}
