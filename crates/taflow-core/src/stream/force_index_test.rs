use super::force_index::ForceIndex;

#[test]
fn lifecycle_is_causal_and_resettable() {
    let mut state = ForceIndex::new();
    assert_eq!(state.append(10.0, 2.0), None);
    assert_eq!(state.append(11.0, 2.0), Some(2.0));
    state.reset();
    assert_eq!(state.value(), None);
}
