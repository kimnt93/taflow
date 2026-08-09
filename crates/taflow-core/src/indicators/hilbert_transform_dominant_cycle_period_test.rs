use super::hilbert_transform_dominant_cycle_period::HilbertTransformDominantCyclePeriod;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = HilbertTransformDominantCyclePeriod::new();
    for value in (0..80).map(|i| 100.0 + i as f64 * 0.1) {
        state.append(value);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
