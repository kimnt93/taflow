use super::hull_moving_average::HullMovingAverage;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = HullMovingAverage::new(9).unwrap();
    for value in 0..12 {
        state.append(value as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
