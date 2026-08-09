use super::rolling_linear_regression_angle::RollingLinearRegressionAngle;
use super::StreamingIndicator;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = RollingLinearRegressionAngle::new(3).unwrap();
    assert!(state.append(1.0).is_none());
    assert!(state.append(2.0).is_none());
    assert!(state.append(3.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
