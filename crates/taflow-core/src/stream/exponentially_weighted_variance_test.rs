use super::exponentially_weighted_variance::ExponentiallyWeightedVariance;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let input = [1.0, 2.0, 4.0, 3.0];
    let mut state = ExponentiallyWeightedVariance::new(4).unwrap();
    let first: Vec<f64> = input.iter().map(|&value| state.append(value)).collect();
    state.reset();
    let replay: Vec<f64> = input.iter().map(|&value| state.append(value)).collect();
    assert_eq!(first, replay);
    assert_eq!(state.value(), replay.last().copied());
}
