use super::moving_average_convergence_divergence_extended::MovingAverageConvergenceDivergenceExtended;
use crate::ma_type::MaType;

#[test]
fn extended_macd_bulk_matches_scalar() {
    let input: Vec<f64> = (0..384)
        .map(|index| 90.0 + (index as f64 * 0.11).sin() * 5.0)
        .collect();
    let mut scalar = MovingAverageConvergenceDivergenceExtended::new(
        12,
        MaType::ExponentialMovingAverage,
        26,
        MaType::ExponentialMovingAverage,
        9,
        MaType::ExponentialMovingAverage,
    )
    .unwrap();
    let mut expected = (Vec::new(), Vec::new(), Vec::new());
    for value in input.iter().copied() {
        if let Some(value) = scalar.append(value) {
            expected.0.push(value.macd);
            expected.1.push(value.signal);
            expected.2.push(value.histogram);
        } else {
            expected.0.push(f64::NAN);
            expected.1.push(f64::NAN);
            expected.2.push(f64::NAN);
        }
    }
    let mut bulk = MovingAverageConvergenceDivergenceExtended::new(
        12,
        MaType::ExponentialMovingAverage,
        26,
        MaType::ExponentialMovingAverage,
        9,
        MaType::ExponentialMovingAverage,
    )
    .unwrap();
    let mut actual = (Vec::new(), Vec::new(), Vec::new());
    bulk.extend_slices_into(&input, &mut actual.0, &mut actual.1, &mut actual.2);
    for (actual, expected) in actual.0.iter().zip(&expected.0) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    for (actual, expected) in actual.1.iter().zip(&expected.1) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    for (actual, expected) in actual.2.iter().zip(&expected.2) {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}
