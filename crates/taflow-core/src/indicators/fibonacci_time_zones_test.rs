use super::fibonacci_time_zones::FibonacciTimeZones;

#[test]
fn lifecycle_is_aligned_and_resettable() {
    let mut state = FibonacciTimeZones::new().unwrap();
    state.append(10.0, 5.0);
    state.append(11.0, 6.0);
    assert_eq!(state.len(), 2);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
