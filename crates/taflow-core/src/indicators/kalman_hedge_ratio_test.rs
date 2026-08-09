use super::KalmanHedgeRatio;

#[test]
fn kalman_hedge_ratio_replays_after_reset() {
    let x: Vec<f64> = (0..64).map(|i| i as f64 / 8.0).collect();
    let y: Vec<f64> = x.iter().map(|v| 1.0 + 2.0 * v).collect();
    let mut state = KalmanHedgeRatio::new(1e-4, 1e-3).unwrap();
    let first: Vec<f64> = x
        .iter()
        .zip(&y)
        .map(|(&a, &b)| state.append(a, b).unwrap())
        .collect();
    assert!(first.last().copied().unwrap() > 1.5);
    assert!(state.alpha().is_some());
    state.reset();
    let second: Vec<f64> = x
        .iter()
        .zip(&y)
        .map(|(&a, &b)| state.append(a, b).unwrap())
        .collect();
    assert_eq!(
        first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
}
