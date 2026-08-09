use super::gap_down::GapDown;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = GapDown::new();
    assert_eq!(state.append(10.0, 8.0), None);
    assert_eq!(state.append(7.0, 6.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
