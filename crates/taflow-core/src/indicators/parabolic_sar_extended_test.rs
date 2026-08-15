use super::parabolic_sar_extended::ParabolicSarExtended;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = ParabolicSarExtended::new(0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2);
    for index in 0..8 {
        state.append(101.0 + index as f64, 99.0 + index as f64);
    }
    let value = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for index in 0..8 {
        state.append(101.0 + index as f64, 99.0 + index as f64);
    }
    assert_eq!(state.value(), value);
}

#[test]
fn bulk_chunks_and_continuation_match_scalar_bits_for_parameter_matrix() {
    let bars = (0..513)
        .map(|index| {
            let phase = index % 58;
            let center = if phase < 29 {
                77.0 + phase as f64 * 1.31
            } else {
                115.0 - (phase - 29) as f64 * 1.37
            };
            let spread = 0.65 + (index % 9) as f64 * 0.08;
            (center + spread, center - spread)
        })
        .collect::<Vec<_>>();
    let high = bars.iter().map(|bar| bar.0).collect::<Vec<_>>();
    let low = bars.iter().map(|bar| bar.1).collect::<Vec<_>>();
    let configurations = [
        (0.0, 0.0, 0.02, 0.02, 0.2, 0.02, 0.02, 0.2),
        (88.0, 0.015, 0.03, 0.017, 0.16, 0.025, 0.019, 0.14),
        (-112.0, 0.025, 0.025, 0.021, 0.18, 0.04, 0.016, 0.13),
    ];

    for configuration in configurations {
        let create = || {
            ParabolicSarExtended::new(
                configuration.0,
                configuration.1,
                configuration.2,
                configuration.3,
                configuration.4,
                configuration.5,
                configuration.6,
                configuration.7,
            )
        };
        let mut scalar = create();
        let expected = high
            .iter()
            .zip(&low)
            .map(|(&high, &low)| scalar.append(high, low).unwrap_or(f64::NAN).to_bits())
            .collect::<Vec<_>>();

        let mut bulk = create();
        let mut bulk_output = vec![123.0];
        bulk.extend_slice_into(&high, &low, &mut bulk_output);
        assert_eq!(
            bulk_output[1..]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            bulk.value().unwrap().to_bits(),
            scalar.value().unwrap().to_bits()
        );

        for index in 513..700 {
            let phase = index % 43;
            let center = if phase < 22 {
                89.0 + phase as f64 * 0.97
            } else {
                110.0 - (phase - 22) as f64 * 1.03
            };
            let high = center + 1.4;
            let low = center - 1.2;
            assert_eq!(
                bulk.append(high, low).unwrap().to_bits(),
                scalar.append(high, low).unwrap().to_bits()
            );
        }

        let mut chunked = create();
        let mut chunked_output = Vec::new();
        chunked.extend_slice_into(&high[..1], &low[..1], &mut chunked_output);
        chunked.extend_slice_into(&high[1..149], &low[1..149], &mut chunked_output);
        chunked.extend_slice_into(&high[149..], &low[149..], &mut chunked_output);
        assert_eq!(
            chunked_output
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
        );

        chunked.reset();
        let mut replay = Vec::new();
        chunked.extend_slice_into(&high, &low, &mut replay);
        assert_eq!(
            replay
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            expected
        );
    }
}
