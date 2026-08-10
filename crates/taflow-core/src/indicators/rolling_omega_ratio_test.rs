use super::rolling_omega_ratio::RollingOmegaRatio;
#[test]
fn lifecycle() {
    let mut state = RollingOmegaRatio::new(3, 0.0).unwrap();
    assert!(state.append(1.0).is_none());
    assert!(state.append(1.0).is_none());
    assert_eq!(state.append(-1.0), Some(2.0));
    state.reset();
    assert!(state.value().is_none());
}
