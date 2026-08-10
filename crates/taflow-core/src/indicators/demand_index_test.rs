use super::demand_index::DemandIndex;
#[test]
fn lifecycle() {
    let mut state = DemandIndex::new().unwrap();
    assert!(state.append(2.0, 0.0, 1.0, 10.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
