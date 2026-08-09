use super::parabolic_sar_extended::ParabolicSarExtended;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = ParabolicSarExtended::new(0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2);
    for index in 0..8 {
        state.append(101.0 + index as f64, 99.0 + index as f64);
    }
    let value = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for index in 0..8 {
        state.append(101.0 + index as f64, 99.0 + index as f64);
    }
    assert_eq!(state.value(), value);
}
