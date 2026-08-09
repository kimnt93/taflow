use super::signal_delay::SignalDelay;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = SignalDelay::new(2).unwrap();
    assert_eq!(state.append(1.0), None);
    assert_eq!(state.append(2.0), None);
    assert_eq!(state.append(3.0), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
