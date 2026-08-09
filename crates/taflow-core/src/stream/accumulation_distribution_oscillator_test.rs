use super::accumulation_distribution_oscillator::AccumulationDistributionOscillator;

fn series(length: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<_> = (0..length)
        .map(|index| 100.0 + index as f64 * 0.1 + (index as f64 * 0.31).sin())
        .collect();
    let high = close.iter().map(|value| value + 1.25).collect();
    let low = close.iter().map(|value| value - 0.75).collect();
    let volume = (0..length)
        .map(|index| 1_000.0 + (index % 17) as f64 * 31.0)
        .collect();
    (high, low, close, volume)
}

#[test]
fn scalar_chunked_reset_and_continuation_are_bitwise_identical() {
    let (high, low, close, volume) = series(257);
    for (fast, slow) in [(2, 3), (3, 10), (12, 5)] {
        let mut scalar_state = AccumulationDistributionOscillator::new(fast, slow).unwrap();
        let scalar: Vec<_> = (0..high.len())
            .map(|index| {
                scalar_state
                    .append(high[index], low[index], close[index], volume[index])
                    .unwrap_or(f64::NAN)
            })
            .collect();
        assert!(scalar[..fast.max(slow) - 1]
            .iter()
            .all(|value| value.is_nan()));

        scalar_state.reset();
        let replay: Vec<_> = (0..high.len())
            .map(|index| {
                scalar_state
                    .append(high[index], low[index], close[index], volume[index])
                    .unwrap_or(f64::NAN)
            })
            .collect();
        for (actual, expected) in replay.iter().zip(&scalar) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }

        let mut bulk_state = AccumulationDistributionOscillator::new(fast, slow).unwrap();
        let mut bulk = Vec::new();
        for range in [0..17, 17..91, 91..257] {
            bulk_state
                .extend_slices_into(
                    &high[range.clone()],
                    &low[range.clone()],
                    &close[range.clone()],
                    &volume[range],
                    &mut bulk,
                )
                .unwrap();
        }
        for (actual, expected) in bulk.iter().zip(&scalar) {
            assert_eq!(actual.to_bits(), expected.to_bits());
        }
        assert_eq!(bulk_state.value(), scalar_state.value());
        assert_eq!(
            bulk_state.append(101.0, 99.0, 100.5, 2_000.0),
            scalar_state.append(101.0, 99.0, 100.5, 2_000.0)
        );
    }
}

#[test]
fn configuration_and_lengths_are_validated_before_mutation() {
    assert!(AccumulationDistributionOscillator::new(1, 10).is_err());
    assert!(AccumulationDistributionOscillator::new(3, 1).is_err());
    let mut state = AccumulationDistributionOscillator::new(3, 10).unwrap();
    let mut output = Vec::new();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &[1.0, 2.0], &mut output)
        .is_err());
    assert!(output.is_empty());
    assert_eq!(state.value(), None);
}
