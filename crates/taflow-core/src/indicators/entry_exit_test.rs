use super::entry_exit::EntryExit;

#[test]
fn tracks_entry_exit_transitions_and_resets() {
    let mut state = EntryExit::new();
    assert_eq!(state.append(false, false), 0.0);
    assert_eq!(state.append(true, false), 1.0);
    assert_eq!(state.append(false, true), -1.0);
    assert_eq!(state.append(true, true), -1.0);
    state.reset();
    assert_eq!(state.value(), None);
}
