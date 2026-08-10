use super::fibonacci_arcs::FibonacciArcs;

#[test]
fn lifecycle_is_aligned_and_resettable() {
    let mut state = FibonacciArcs::new().unwrap();
    for index in 0..2 {
        state.append(10.0 + index as f64, 5.0 - index as f64);
    }
    assert_eq!(state.len(), 2);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
