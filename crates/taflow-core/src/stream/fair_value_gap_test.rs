use super::fair_value_gap::FairValueGap;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = FairValueGap::new();
    assert!(state.append(10.0, 11.0, 9.0, 10.5).is_some());
    state.reset();
    assert!(state.value().is_none());
}
