use super::DrawdownState;
use approx::assert_relative_eq;

#[test]
fn tracks_phantom_wealth_peak_and_nonpositive_drawdown() {
    let mut state = DrawdownState::new();
    for value in [0.1, -0.2, 0.25, -0.5] {
        state.append(value).unwrap();
    }
    assert_relative_eq!(state.wealth(), 0.55, epsilon = 1e-15);
    assert_relative_eq!(state.peak(), 1.1, epsilon = 1e-15);
    assert_relative_eq!(state.current_drawdown().unwrap(), -0.5, epsilon = 1e-15);
    assert_relative_eq!(state.maximum_drawdown().unwrap(), -0.5, epsilon = 1e-15);
    state.reset();
    assert_eq!(state.maximum_drawdown(), None);
}
