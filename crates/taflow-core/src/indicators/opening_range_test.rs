use super::opening_range::OpeningRange;

#[test]
fn reset_replay_matches() {
    let high: Vec<f64> = (0..32).map(|index| 100.0 + index as f64).collect();
    let low: Vec<f64> = high.iter().map(|value| value - 2.0).collect();
    let close = high.clone();
    let anchor = vec![false; high.len()];
    let mut state = OpeningRange::new(5);
    let mut highs = Vec::new();
    let mut lows = Vec::new();
    let mut breakouts = Vec::new();
    state
        .extend_slice_into(
            &high,
            &low,
            &close,
            &anchor,
            &mut highs,
            &mut lows,
            &mut breakouts,
        )
        .unwrap();
    let final_value = state.value();
    state.reset();
    let mut h = Vec::new();
    let mut l = Vec::new();
    let mut b = Vec::new();
    state
        .extend_slice_into(&high, &low, &close, &anchor, &mut h, &mut l, &mut b)
        .unwrap();
    assert_eq!(highs, h);
    assert_eq!(lows, l);
    assert_eq!(breakouts, b);
    assert_eq!(state.value(), final_value);
}
