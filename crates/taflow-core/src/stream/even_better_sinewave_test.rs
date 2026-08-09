use super::even_better_sinewave::EvenBetterSinewave;
use super::StreamingIndicator;

#[test]
fn bulk_and_reset_replay_match() {
    let input: Vec<f64> = (0..128)
        .map(|index| 100.0 + (index as f64 / 8.0).sin())
        .collect();
    let mut state = EvenBetterSinewave::new(40).unwrap();
    let mut first = Vec::new();
    state.extend_slice_into(&input, &mut first);
    assert!(first[..39].iter().all(|value| value.is_nan()));
    let final_value = state.value();
    state.reset();
    let mut second = Vec::new();
    state.extend_slice_into(&input, &mut second);
    for (left, right) in first.iter().zip(second) {
        assert_eq!(left.to_bits(), right.to_bits());
    }
    assert_eq!(state.value(), final_value);
}
