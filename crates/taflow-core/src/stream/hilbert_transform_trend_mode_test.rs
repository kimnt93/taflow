use super::hilbert_transform_trend_mode::HilbertTransformTrendMode;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = HilbertTransformTrendMode::new();
    for value in (0..100).map(|i| 100.0 + (i as f64 * 0.1).sin()) {
        state.append(value);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
