use super::ease_of_movement::EaseOfMovement;

#[test]
fn lifecycle_is_causal_and_resettable() {
    let mut state = EaseOfMovement::new();
    assert_eq!(state.append(11.0, 9.0, 2.0), None);
    assert_eq!(state.append(12.0, 10.0, 2.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
