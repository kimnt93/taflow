use super::session_volume_levels::SessionVolumeLevels;

#[test]
fn reset_replay_matches() {
    let high = vec![110.0; 32];
    let low = vec![90.0; 32];
    let close = vec![100.0; 32];
    let volume = vec![1000.0; 32];
    let anchor = vec![false; 32];
    let mut state = SessionVolumeLevels::new(24, 0.7).unwrap();
    let mut poc = Vec::new();
    let mut vah = Vec::new();
    let mut val = Vec::new();
    state
        .extend_slice_into(
            &high, &low, &close, &volume, &anchor, &mut poc, &mut vah, &mut val,
        )
        .unwrap();
    let final_value = state.value();
    state.reset();
    let mut p = Vec::new();
    let mut h = Vec::new();
    let mut l = Vec::new();
    state
        .extend_slice_into(
            &high, &low, &close, &volume, &anchor, &mut p, &mut h, &mut l,
        )
        .unwrap();
    assert_eq!(poc, p);
    assert_eq!(vah, h);
    assert_eq!(val, l);
    assert_eq!(state.value(), final_value);
}
