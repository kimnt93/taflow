use super::chaikin_volatility::ChaikinVolatility;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = ChaikinVolatility::new(3, 2).unwrap();
    for i in 0..7 {
        state.append(i as f64 + 10.0, i as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
