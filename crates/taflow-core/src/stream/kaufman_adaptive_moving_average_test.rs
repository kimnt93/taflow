use super::kaufman_adaptive_moving_average::KaufmanAdaptiveMovingAverage;
use super::StreamingIndicator;

#[test]
fn bulk_and_reset_replay_match() {
    let input: Vec<f64> = (0..160)
        .map(|index| 100.0 + (index as f64 * 0.17).sin() * 3.0)
        .collect();
    let mut scalar = KaufmanAdaptiveMovingAverage::new(10).unwrap();
    let expected: Vec<f64> = input
        .iter()
        .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
        .collect();
    let mut bulk = KaufmanAdaptiveMovingAverage::new(10).unwrap();
    let mut actual = Vec::new();
    bulk.extend_slice_into(&input, &mut actual);
    for (expected, actual) in expected.iter().zip(&actual) {
        assert!((expected.is_nan() && actual.is_nan()) || expected.to_bits() == actual.to_bits());
    }
    bulk.reset();
    let mut replay = Vec::new();
    bulk.extend_slice_into(&input, &mut replay);
    for (expected, replay) in expected.iter().zip(&replay) {
        assert!((expected.is_nan() && replay.is_nan()) || expected.to_bits() == replay.to_bits());
    }
}
