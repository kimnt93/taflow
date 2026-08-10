use super::rolling_treynor_ratio::RollingTreynorRatio;
#[test]
fn lifecycle() {
    let mut state = RollingTreynorRatio::new(3).unwrap();
    state.append(1.0, 1.0);
    state.append(2.0, 2.0);
    assert!(state.append(3.0, 3.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
