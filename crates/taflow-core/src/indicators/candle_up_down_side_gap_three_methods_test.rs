use super::candle_up_down_side_gap_three_methods::CandleUpDownSideGapThreeMethods;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleUpDownSideGapThreeMethods::new();
    for index in 0..8 {
        state.append(
            100.0 + index as f64,
            102.0 + index as f64,
            99.0 + index as f64,
            101.0 + index as f64,
        );
    }
    let expected = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for index in 0..8 {
        state.append(
            100.0 + index as f64,
            102.0 + index as f64,
            99.0 + index as f64,
            101.0 + index as f64,
        );
    }
    assert_eq!(state.value(), expected);
}
