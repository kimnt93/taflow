use super::aroon_oscillator::AroonOscillator;

#[test]
fn scalar_bulk_reset_and_validation_are_bitwise_identical() {
    let high: Vec<_> = (0..2_003)
        .map(|index| ((index * 11) % 17) as f64 + (index as f64 * 0.13).sin())
        .collect();
    let low: Vec<_> = high.iter().map(|value| value - 2.0).collect();
    for period in [2_usize, 5, 14, 30, 200] {
        let mut scalar_state = AroonOscillator::new(period).unwrap();
        let scalar: Vec<_> = (0..high.len())
            .map(|index| {
                scalar_state
                    .append(high[index], low[index])
                    .unwrap_or(f64::NAN)
            })
            .collect();

        for chunk in [1_usize, 10, 97, high.len()] {
            let mut bulk_state = AroonOscillator::new(period).unwrap();
            let mut bulk = Vec::new();
            let mut offset = 0;
            while offset < high.len() {
                let end = (offset + chunk).min(high.len());
                bulk_state
                    .extend_slices_into(&high[offset..end], &low[offset..end], &mut bulk)
                    .unwrap();
                offset = end;
            }
            for (actual, expected) in bulk.iter().zip(&scalar) {
                assert_eq!(actual.to_bits(), expected.to_bits());
            }
            assert_eq!(bulk_state.value(), scalar_state.value());
        }

        scalar_state.reset();
        assert_eq!(scalar_state.value(), None);
        let replay: Vec<_> = (0..high.len())
            .map(|index| {
                scalar_state
                    .append(high[index], low[index])
                    .unwrap_or(f64::NAN)
            })
            .collect();
        for (actual, expected) in replay.iter().zip(&scalar) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
    }

    assert!(AroonOscillator::new(1).is_err());
    let mut state = AroonOscillator::new(5).unwrap();
    let mut output = Vec::new();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &mut output)
        .is_err());
    assert!(output.is_empty());
}
