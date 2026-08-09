use super::sessions::Sessions;

#[test]
fn session_boundaries_and_reset_are_consistent() {
    let mut state = Sessions::new();
    assert_eq!(state.append(true, 10.0, 8.0).session_high, 10.0);
    assert_eq!(state.append(false, 12.0, 7.0).session_low, 7.0);
    state.reset();
    assert!(state.value().is_none());
}
