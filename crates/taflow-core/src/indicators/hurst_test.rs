use super::hurst::Hurst;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = Hurst::new(8, 4).unwrap();
    for value in 1..8 {
        assert_eq!(state.append(value as f64), None);
    }
    assert!(state.append(8.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn reset_replay_is_bitwise_invariant() {
    let input: Vec<f64> = (0..173)
        .map(|index| 50.0 + (index as f64 * 0.137).sin() * 2.0 + index as f64 * 0.011)
        .collect();
    let mut scalar = Hurst::new(20, 4).unwrap();
    let expected: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    scalar.reset();
    let actual: Vec<_> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    for (&actual, &expected) in actual.iter().zip(&expected) {
        assert!(actual.to_bits() == expected.to_bits() || (actual.is_nan() && expected.is_nan()));
    }
}
