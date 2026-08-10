use super::keltner_channels::KeltnerChannels;

#[test]
fn scalar_outputs_and_reset_are_consistent() {
    let mut state = KeltnerChannels::new(2, 2.0).unwrap();
    assert_eq!(state.append(12.0, 8.0, 10.0), None);
    let first = state.append(13.0, 9.0, 11.0).unwrap();
    assert_eq!(first.middle, 10.5);
    assert_eq!(first.upper, 18.5);
    assert_eq!(first.lower, 2.5);
    state.reset();
    assert_eq!(state.value(), None);
}
