use super::swing_high::SwingHigh;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = SwingHigh::new(2).unwrap();
    state.append(10.0, 8.0);
    state.reset();
    assert!(state.value().is_none());
}
