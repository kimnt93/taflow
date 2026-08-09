use super::bars_since::BarsSince;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = BarsSince::new();
    assert_eq!(state.append(false), Some(0.0));
    assert_eq!(state.append(false), Some(1.0));
    assert_eq!(state.append(true), Some(0.0));
    state.reset();
    assert_eq!(state.value(), None);
}
