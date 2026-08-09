use super::ulcer_index::UlcerIndex;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = UlcerIndex::new(4).unwrap();
    for value in [10.0, 9.0, 8.0, 9.0, 7.0] {
        state.append(value);
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
