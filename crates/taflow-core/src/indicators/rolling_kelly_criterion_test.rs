use super::rolling_kelly_criterion::RollingKellyCriterion;
#[test]
fn lifecycle() {
    let mut state = RollingKellyCriterion::new(3).unwrap();
    state.append(1.0);
    state.append(-1.0);
    assert!(state.append(1.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
