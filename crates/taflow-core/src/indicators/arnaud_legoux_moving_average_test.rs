use super::arnaud_legoux_moving_average::ArnaudLegouxMovingAverage;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = ArnaudLegouxMovingAverage::new(5, 0.85, 6.0).unwrap();
    for value in 0..8 {
        state.append(value as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
