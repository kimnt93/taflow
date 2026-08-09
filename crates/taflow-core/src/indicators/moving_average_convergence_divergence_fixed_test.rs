use super::moving_average_convergence_divergence_fixed::MovingAverageConvergenceDivergenceFixed;

#[test]
fn fixed_macd_warms_and_reset_replays() {
    let input: Vec<f64> = (0..256)
        .map(|index| 100.0 + (index as f64 * 0.23).cos() * 3.0)
        .collect();
    let mut state = MovingAverageConvergenceDivergenceFixed::new(9).unwrap();
    assert!(state.value().is_none());
    for value in input.iter().copied() {
        state.append(value);
    }
    let expected = state.value();
    assert!(expected.is_some());
    state.reset();
    assert!(state.value().is_none());
    for value in input {
        state.append(value);
    }
    assert_eq!(state.value(), expected);
}
