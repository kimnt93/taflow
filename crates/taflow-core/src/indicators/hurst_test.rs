use super::hurst::Hurst;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = Hurst::new(8, 4).unwrap();
    for value in 1..8 {
        assert_eq!(state.append(value as f64), None);
    }
    assert!(state.append(8.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
