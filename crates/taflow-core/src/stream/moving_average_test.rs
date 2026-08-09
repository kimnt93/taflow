use super::moving_average::MovingAverage;
use super::StreamingIndicator;
use crate::ma_type::MaType;

#[test]
fn matches_scalar_lifecycle_for_all_moving_average_types() {
    let input: Vec<f64> = (0..200)
        .map(|index| 100.0 + (index as f64 * 0.29).sin() * 6.0 + index as f64 * 0.02)
        .collect();
    for code in 0..=8 {
        let ma_type = MaType::try_from(code).unwrap();
        let mut scalar = MovingAverage::new(13, ma_type).unwrap();
        let expected: Vec<_> = input
            .iter()
            .map(|&value| scalar.append(value).unwrap_or(f64::NAN))
            .collect();
        let mut batch = MovingAverage::new(13, ma_type).unwrap();
        let mut actual = Vec::new();
        batch.extend_slice_into(&input, &mut actual);
        assert!(
            actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.to_bits() == expected.to_bits()),
            "MA type {code}"
        );
    }
}

#[test]
fn period_one_is_identity() {
    let input = [1.0, 3.0, 2.0, 8.0];
    for code in 0..=8 {
        let ma_type = MaType::try_from(code).unwrap();
        let mut state = MovingAverage::new(1, ma_type).unwrap();
        let mut output = Vec::new();
        state.extend_slice_into(&input, &mut output);
        assert_eq!(output, input);
    }
}
