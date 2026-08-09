use super::gap_up::GapUp;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = GapUp::new();
    assert_eq!(state.append(10.0, 8.0), None);
    assert_eq!(state.append(12.0, 11.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
