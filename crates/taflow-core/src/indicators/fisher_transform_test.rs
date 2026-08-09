use super::fisher_transform::FisherTransform;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = FisherTransform::new(5).unwrap();
    for i in 0..8 {
        state.append(i as f64 + 1.0, i as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
