use super::automatic_fibonacci::AutomaticFibonacci;

#[test]
fn lifecycle_is_aligned_and_resettable() {
    let mut state = AutomaticFibonacci::new().unwrap();
    for index in 0..2 {
        state.append(10.0 + index as f64, 5.0 - index as f64);
    }
    assert_eq!(state.len(), 2);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
