use super::OnlineMoments;
use approx::assert_relative_eq;

#[test]
fn computes_sample_and_population_moments_and_resets() {
    let mut state = OnlineMoments::new();
    for value in [1.0, 2.0, 3.0, 4.0] {
        state.append(value);
    }
    assert_eq!(state.mean(), Some(2.5));
    assert_eq!(state.variance(0), Some(1.25));
    assert_relative_eq!(state.variance(1).unwrap(), 5.0 / 3.0);
    assert_eq!(state.variance(4), None);
    state.reset();
    assert!(state.is_empty());
    assert_eq!(state.mean(), None);
}
