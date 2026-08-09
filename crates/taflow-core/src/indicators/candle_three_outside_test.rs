use super::candle_three_outside::CandleThreeOutside;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleThreeOutside::new();
    for index in 0..16 {
        let price = 100.0 + index as f64;
        state.append(price, price + 2.0, price - 2.0, price + 0.5);
    }
    state.reset();
    assert_eq!(state.value(), None);
}
