use super::hedge_ratio::HedgeRatio;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = HedgeRatio::new(2).unwrap();
    assert_eq!(state.append(1.0, 2.0), None);
    assert_eq!(state.append(2.0, 4.0), Some(2.0));
    state.reset();
    assert_eq!(state.value(), None);
}
