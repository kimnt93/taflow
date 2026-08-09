use super::crossover::Crossover;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = Crossover::new();
    assert_eq!(state.append(1.0, 2.0), 0.0);
    assert_eq!(state.append(3.0, 2.0), 1.0);
    assert_eq!(state.value(), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
