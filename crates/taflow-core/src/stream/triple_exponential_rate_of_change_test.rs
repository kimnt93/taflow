use super::triple_exponential_rate_of_change::TripleExponentialRateOfChange;
use super::StreamingIndicator;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = TripleExponentialRateOfChange::new(7).unwrap();
    for value in (0..40).map(|i| 100.0 + i as f64) {
        state.append(value);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
