use super::drawdown::Drawdown;

#[test]
fn running_maximum_and_reset_are_consistent() {
    let mut state = Drawdown::new();
    assert_eq!(state.append(10.0), 0.0);
    assert!((state.append(8.0) + 0.2).abs() < 1e-12);
    state.reset();
    assert_eq!(state.value(), None);
    assert_eq!(state.append(10.0), 0.0);
}
