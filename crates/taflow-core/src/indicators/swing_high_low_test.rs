use super::swing_high_low::SwingHighLow;

#[test]
fn reset_replays_empty_state() {
    let mut state = SwingHighLow::new(2).unwrap();
    assert!(state.append(10.0, 8.0).is_none());
    state.reset();
    assert!(state.value().is_none());
}
