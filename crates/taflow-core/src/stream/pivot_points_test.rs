use super::pivot_points::PivotPoints;

#[test]
fn reset_replay_matches() {
    let high = vec![110.0; 20];
    let low = vec![90.0; 20];
    let close = vec![100.0; 20];
    let anchor = vec![
        true, false, false, true, false, false, true, false, false, true, false, false, true,
        false, false, true, false, false, true, false,
    ];
    let mut state = PivotPoints::new();
    let mut output = std::array::from_fn(|_| Vec::new());
    state
        .extend_slice_into(&high, &low, &close, &anchor, &mut output)
        .unwrap();
    let final_value = state.value();
    state.reset();
    let mut replay = std::array::from_fn(|_| Vec::new());
    state
        .extend_slice_into(&high, &low, &close, &anchor, &mut replay)
        .unwrap();
    assert_eq!(output, replay);
    assert_eq!(state.value(), final_value);
}
