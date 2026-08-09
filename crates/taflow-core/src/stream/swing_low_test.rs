use super::swing_low::SwingLow;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = SwingLow::new(2).unwrap();
    state.append(10.0, 8.0);
    state.reset();
    assert!(state.value().is_none());
}
