use super::plus_directional_indicator::PlusDirectionalIndicator;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = PlusDirectionalIndicator::new(3).unwrap();
    for index in 0..8 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    let value = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for index in 0..8 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    assert_eq!(state.value(), value);
}
