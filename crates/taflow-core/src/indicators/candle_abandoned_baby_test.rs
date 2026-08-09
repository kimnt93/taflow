use super::candle_abandoned_baby::CandleAbandonedBaby;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleAbandonedBaby::default();
    for index in 0..12 {
        state.append(
            100.0 + index as f64,
            102.0 + index as f64,
            98.0 + index as f64,
            101.0 + index as f64,
        );
    }
    state.reset();
    assert_eq!(state.value(), None);
}
