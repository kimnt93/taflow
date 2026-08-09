use super::jurik_moving_average::JurikMovingAverage;
use super::StreamingIndicator;

#[test]
fn bulk_and_reset_replay_match() {
    let input: Vec<f64> = (0..128).map(|index| 100.0 + index as f64 * 0.2).collect();
    let mut state = JurikMovingAverage::new(7, 0.0).unwrap();
    let mut first = Vec::new();
    state.extend_slice_into(&input, &mut first);
    let final_value = state.value();
    state.reset();
    let mut second = Vec::new();
    state.extend_slice_into(&input, &mut second);
    assert_eq!(first.len(), second.len());
    for (first, second) in first.iter().zip(second) {
        assert_eq!(first.to_bits(), second.to_bits());
    }
    assert_eq!(state.value(), final_value);
}
