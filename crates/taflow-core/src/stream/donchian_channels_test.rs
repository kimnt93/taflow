use super::donchian_channels::DonchianChannels;

#[test]
fn lifecycle_and_reset_are_consistent() {
    let mut state = DonchianChannels::new(2).unwrap();
    assert!(state.append(10.0, 8.0).is_none());
    assert!(state.append(12.0, 7.0).is_some());
    state.reset();
    assert!(state.value().is_none());
}
