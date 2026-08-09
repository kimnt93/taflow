use super::rolling_min_max_index::RollingMinMaxIndex;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingMinMaxIndex::new(3).unwrap();
    let _ = state.append(1.0);
    let _ = state.append(2.0);
    let _ = state.append(3.0);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.value().is_none());
}
