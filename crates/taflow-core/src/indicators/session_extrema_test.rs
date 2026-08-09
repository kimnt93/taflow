use super::session_extrema::SessionExtrema;

#[test]
fn session_boundaries_and_reset_are_consistent() {
    let mut state = SessionExtrema::new();
    assert_eq!(state.append(true, 10.0, 8.0).high, 10.0);
    assert_eq!(state.append(false, 12.0, 7.0).low, 7.0);
    state.reset();
    assert!(state.value().is_none());
}
