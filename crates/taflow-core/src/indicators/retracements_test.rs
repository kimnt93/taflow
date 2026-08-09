use super::retracements::Retracements;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = Retracements::new(2).unwrap();
    assert!(state.append(10.0, 8.0, 9.0).direction.is_nan());
    state.reset();
    assert!(state.value().is_none());
}
