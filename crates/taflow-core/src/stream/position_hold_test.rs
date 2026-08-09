use super::position_hold::PositionHold;

#[test]
fn holds_non_zero_positions_and_resets() {
    let mut state = PositionHold::new();
    assert_eq!(state.append(0.0), 0.0);
    assert_eq!(state.append(2.0), 2.0);
    assert_eq!(state.append(0.0), 2.0);
    state.reset();
    assert_eq!(state.value(), None);
}
