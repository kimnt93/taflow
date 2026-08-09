use super::previous_high_low::PreviousHighLow;

#[test]
fn session_breaks_and_reset_are_consistent() {
    let mut state = PreviousHighLow::new();
    state.append(true, 10.0, 8.0);
    let value = state.append(true, 12.0, 7.0);
    assert_eq!(value.prev_high, 10.0);
    state.reset();
    assert!(state.value().is_none());
}
