use super::smoothed_trend_channel::SmoothedTrendChannel;

#[test]
fn warmup_and_reset_are_causal() {
    let mut state = SmoothedTrendChannel::new(2).unwrap();
    assert_eq!(state.append(10.0, 8.0, 9.0), None);
    assert_eq!(state.append(12.0, 9.0, 11.0), Some((8.5, 11.0)));
    state.reset();
    assert_eq!(state.value(), None);
}
