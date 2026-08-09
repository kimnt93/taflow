use super::higher_high::HigherHigh;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = HigherHigh::new();
    assert_eq!(state.append(10.0, 8.0), None);
    assert_eq!(state.append(11.0, 9.0), Some(1.0));
    assert_eq!(state.value(), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
