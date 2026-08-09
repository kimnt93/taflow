use super::cross::Cross;

#[test]
fn detects_both_cross_directions_and_resets() {
    let mut state = Cross::new();
    assert_eq!(state.append(0.0, 1.0), 0.0);
    assert_eq!(state.append(2.0, 1.0), 1.0);
    assert_eq!(state.append(0.0, 1.0), 1.0);
    state.reset();
    assert_eq!(state.value(), None);
}
