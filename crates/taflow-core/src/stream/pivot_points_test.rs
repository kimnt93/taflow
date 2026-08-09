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
    for (expected, actual) in output.iter().zip(&replay) {
        assert_eq!(expected.len(), actual.len());
        for (expected, actual) in expected.iter().zip(actual) {
            assert_eq!(expected.to_bits(), actual.to_bits());
        }
    }
    assert_eq!(state.value(), final_value);
}
