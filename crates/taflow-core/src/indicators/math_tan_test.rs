use super::math_tan::MathTan;

#[test]
fn lifecycle_and_bulk_are_consistent() {
    let input = [-0.75_f64, -0.25, 0.25, 0.75];
    let expected: Vec<_> = input.iter().map(|&input| input.tan()).collect();
    let mut state = MathTan::new().unwrap();
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

#[test]
fn repeated_bulk_and_scalar_continuation_preserve_exact_state() {
    let input = [-3.0_f64, -0.75, -0.0, 0.25, 1.5, f64::INFINITY, f64::NAN];
    let mut scalar = MathTan::new().unwrap();
    let expected: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap())
        .collect();
    let expected_next = scalar.append(0.125);

    for split in 0..=input.len() {
        let mut chunked = MathTan::new().unwrap();
        let mut actual = vec![17.0];
        chunked.extend_slice_into(&input[..split], &mut actual);
        chunked.extend_slice_into(&input[split..], &mut actual);
        assert_eq!(
            actual[1..]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            chunked.append(0.125).map(f64::to_bits),
            expected_next.map(f64::to_bits)
        );
    }
}
