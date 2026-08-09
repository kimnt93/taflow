use super::FracDiff;

#[test]
fn frac_diff_warms_and_resets() {
    let mut state = FracDiff::new(0.5, 1e-3).unwrap();
    let input: Vec<f64> = (1..=128).map(f64::from).collect();
    let first: Vec<f64> = input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect();
    assert!(first.iter().any(|v| v.is_finite()));
    assert_eq!(
        state.value().map(f64::to_bits),
        first.last().copied().map(f64::to_bits)
    );
    state.reset();
    let second: Vec<f64> = input
        .iter()
        .map(|&v| state.append(v).unwrap_or(f64::NAN))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
}
