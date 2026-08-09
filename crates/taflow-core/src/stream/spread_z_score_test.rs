use super::spread_z_score::SpreadZScore;

#[test]
fn reset_replays_identically() {
    let x: Vec<f64> = (0..100).map(|index| 10.0 + index as f64 * 0.1).collect();
    let y: Vec<f64> = x.iter().map(|value| value * 2.0).collect();
    let mut state = SpreadZScore::new(10).unwrap();
    let first: Vec<u64> = x
        .iter()
        .zip(&y)
        .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN).to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = x
        .iter()
        .zip(&y)
        .map(|(&a, &b)| state.append(a, b).unwrap_or(f64::NAN).to_bits())
        .collect();
    assert_eq!(first, second);
}
