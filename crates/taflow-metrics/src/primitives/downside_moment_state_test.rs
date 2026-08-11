use super::DownsideMomentState;
use approx::assert_relative_eq;

#[test]
fn averages_clipped_shortfalls_over_every_observation() {
    let mut state = DownsideMomentState::new(0.0);
    for value in [-2.0, 1.0, 0.0, -1.0] {
        state.append(value);
    }
    assert_relative_eq!(state.mean_squared_shortfall().unwrap(), 1.25);
    assert_relative_eq!(state.downside_deviation().unwrap(), 1.25_f64.sqrt());
    state.reset();
    assert_eq!(state.downside_deviation(), None);
    assert_eq!(state.required_return(), 0.0);
}
