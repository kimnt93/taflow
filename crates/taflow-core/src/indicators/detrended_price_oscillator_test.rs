use super::detrended_price_oscillator::DetrendedPriceOscillator;

#[test]
fn reset_replays_identically() {
    let input: Vec<f64> = (0..100).map(|index| 100.0 + index as f64).collect();
    let mut state = DetrendedPriceOscillator::new(10).unwrap();
    let first: Vec<u64> = input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN).to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = input
        .iter()
        .map(|&value| state.append(value).unwrap_or(f64::NAN).to_bits())
        .collect();
    assert_eq!(first, second);
}
