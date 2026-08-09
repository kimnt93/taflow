use super::rolling_information_ratio::RollingInformationRatio;

#[test]
fn reset_replays_identically() {
    let input: Vec<f64> = (0..200).map(|index| index as f64 * 0.01).collect();
    let benchmark: Vec<f64> = input.iter().map(|value| value * 0.8).collect();
    let mut state = RollingInformationRatio::new(20).unwrap();
    let first: Vec<u64> = input
        .iter()
        .zip(&benchmark)
        .map(|(&i, &b)| state.append(i, b).unwrap_or(f64::NAN).to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = input
        .iter()
        .zip(&benchmark)
        .map(|(&i, &b)| state.append(i, b).unwrap_or(f64::NAN).to_bits())
        .collect();
    assert_eq!(first, second);
}
