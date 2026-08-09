use super::close_to_close_sigma::CloseToCloseSigma;

#[test]
fn reset_replays_identically() {
    let closes: Vec<f64> = (0..200).map(|index| 100.0 + index as f64 * 0.2).collect();
    let mut state = CloseToCloseSigma::new(20).unwrap();
    let first: Vec<u64> = closes
        .iter()
        .map(|&close| state.append(close).unwrap_or(f64::NAN).to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = closes
        .iter()
        .map(|&close| state.append(close).unwrap_or(f64::NAN).to_bits())
        .collect();
    assert_eq!(first, second);
}
