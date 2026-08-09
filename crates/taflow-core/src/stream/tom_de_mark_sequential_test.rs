use super::tom_de_mark_sequential::TomDeMarkSequential;

#[test]
fn warmup_and_reset_replay_are_deterministic() {
    let close: Vec<f64> = (0..64).map(|index| 100.0 + index as f64).collect();
    let mut state = TomDeMarkSequential::new();
    let mut buy = Vec::new();
    let mut sell = Vec::new();
    state.extend_slice_into(&close, &mut buy, &mut sell);
    assert_eq!(&buy[..4], &[0, 0, 0, 0]);
    assert!(state.value().is_some());
    let final_value = state.value();
    state.reset();
    let mut replay_buy = Vec::new();
    let mut replay_sell = Vec::new();
    state.extend_slice_into(&close, &mut replay_buy, &mut replay_sell);
    assert_eq!(buy, replay_buy);
    assert_eq!(sell, replay_sell);
    assert_eq!(state.value(), final_value);
}
