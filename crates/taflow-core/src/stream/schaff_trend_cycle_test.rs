use super::schaff_trend_cycle::SchaffTrendCycle;

#[test]
fn reset_replays_identically() {
    let close: Vec<f64> = (0..200)
        .map(|index| 100.0 + (index as f64 * 0.13).sin())
        .collect();
    let mut state = SchaffTrendCycle::new(10, 5, 20, 0.5).unwrap();
    let first: Vec<u64> = close
        .iter()
        .map(|&value| state.append(value).stc.to_bits())
        .collect();
    state.reset();
    let second: Vec<u64> = close
        .iter()
        .map(|&value| state.append(value).stc.to_bits())
        .collect();
    assert_eq!(first, second);
}
