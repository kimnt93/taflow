use super::true_strength_index::TrueStrengthIndex;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = TrueStrengthIndex::new(5, 10).unwrap();
    for value in 0..15 {
        state.append(value as f64);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
