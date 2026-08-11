use super::ExactOrderStatistics;
use approx::assert_relative_eq;

#[test]
fn matches_linear_quantiles_and_empyrical_tail_selection() {
    let mut state = ExactOrderStatistics::with_capacity(5);
    for value in [5.0, 1.0, 4.0, 2.0, 3.0] {
        state.append(value);
    }
    assert_eq!(state.quantile(0.0).unwrap(), Some(1.0));
    assert_relative_eq!(state.quantile(0.375).unwrap().unwrap(), 2.5);
    assert_eq!(state.quantile(1.0).unwrap(), Some(5.0));
    assert_relative_eq!(state.lower_tail_mean(0.5).unwrap().unwrap(), 2.0);
    assert!(state.lower_tail_mean(0.0).is_err());
    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.quantile(0.5).unwrap(), None);
}
