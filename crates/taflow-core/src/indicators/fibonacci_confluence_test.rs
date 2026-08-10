use super::fibonacci_confluence::FibonacciConfluence;

#[test]
fn lifecycle_is_aligned_and_resettable() {
    let mut state = FibonacciConfluence::new().unwrap();
    for index in 0..3 {
        state.append(10.0 + index as f64, 5.0 - index as f64);
    }
    assert_eq!(state.len(), 3);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
