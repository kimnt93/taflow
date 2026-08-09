use super::cumulative_count::CumulativeCount;

#[test]
fn scalar_bulk_and_reset_are_invariant() {
    let input: Vec<f64> = (0..32).map(f64::from).collect();
    let mut scalar = CumulativeCount::new();
    let scalar_out: Vec<f64> = input.iter().map(|&x| scalar.append(x)).collect();
    let mut bulk = CumulativeCount::new();
    let mut bulk_out = Vec::new();
    bulk.extend_slice_into(&input, &mut bulk_out);
    assert_eq!(scalar_out, bulk_out);
    assert_eq!(scalar.value(), bulk.value());
    bulk.reset();
    assert_eq!(bulk.value(), None);
    assert_eq!(bulk.append(1.0), 1.0);
}
