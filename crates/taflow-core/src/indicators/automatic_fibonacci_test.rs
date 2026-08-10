use super::automatic_fibonacci::AutomaticFibonacci;

#[test]
fn lifecycle_is_aligned_and_resettable() {
    let mut state = AutomaticFibonacci::new().unwrap();
    for (high, low) in [
        (100.0, 99.0),
        (110.0, 108.0),
        (104.0, 100.0),
        (107.0, 105.0),
    ] {
        state.append(high, low);
    }
    assert_eq!(state.len(), 4);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
