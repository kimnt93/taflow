use super::value_when::ValueWhen;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = ValueWhen::new();
    assert_eq!(state.append(false, 1.0), None);
    assert_eq!(state.append(true, 2.0), Some(2.0));
    assert_eq!(state.append(false, 3.0), Some(2.0));
    state.reset();
    assert_eq!(state.value(), None);
}
