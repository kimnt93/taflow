use super::rising::Rising;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = Rising::new(2).unwrap();
    assert_eq!(state.append(1.0), None);
    assert_eq!(state.append(2.0), None);
    assert_eq!(state.append(3.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
