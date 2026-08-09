use super::math_ln::MathLn;

#[test]
fn lifecycle_and_bulk_are_consistent() {
    let input = [-0.75_f64, -0.25, 0.25, 0.75];
    let expected: Vec<_> = input.iter().map(|&input| input.ln()).collect();
    let mut state = MathLn::new().unwrap();
    assert!(state.value().is_none());
    let scalar: Vec<_> = input
        .iter()
        .map(|&input| state.append(input).unwrap())
        .collect();
    for (actual, expected) in scalar.iter().zip(&expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    assert_eq!(
        state.value().unwrap().to_bits(),
        expected.last().unwrap().to_bits()
    );
    state.reset();
    assert!(state.value().is_none());
    let mut bulk = Vec::new();
    state.extend_slice_into(&input, &mut bulk);
    for (actual, expected) in bulk.iter().zip(&expected) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}
