use super::parabolic_sar::ParabolicSar;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = ParabolicSar::new(0.02, 0.2);
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
fn bulk_chunks_and_continuation_match_scalar_bits() {
    let bars = (0..513)
        .map(|index| {
            let phase = index % 64;
            let center = if phase < 32 {
                82.0 + phase as f64 * 1.17
            } else {
                119.0 - (phase - 32) as f64 * 1.21
            };
            let spread = 0.7 + (index % 7) as f64 * 0.09;
            (center + spread, center - spread)
        })
        .collect::<Vec<_>>();
    let high = bars.iter().map(|bar| bar.0).collect::<Vec<_>>();
    let low = bars.iter().map(|bar| bar.1).collect::<Vec<_>>();

    let mut scalar = ParabolicSar::new(0.035, 0.17);
    let expected = high
        .iter()
        .zip(&low)
        .map(|(&high, &low)| scalar.append(high, low).unwrap_or(f64::NAN).to_bits())
        .collect::<Vec<_>>();

    let mut bulk = ParabolicSar::new(0.035, 0.17);
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
        let phase = index % 47;
        let center = if phase < 24 {
            91.0 + phase as f64 * 0.83
        } else {
            111.0 - (phase - 24) as f64 * 0.91
        };
        let high = center + 1.3;
        let low = center - 1.1;
        assert_eq!(
            bulk.append(high, low).unwrap().to_bits(),
            scalar.append(high, low).unwrap().to_bits()
        );
    }

    let mut chunked = ParabolicSar::new(0.035, 0.17);
    let mut chunked_output = Vec::new();
    chunked.extend_slice_into(&high[..1], &low[..1], &mut chunked_output);
    chunked.extend_slice_into(&high[1..137], &low[1..137], &mut chunked_output);
    chunked.extend_slice_into(&high[137..], &low[137..], &mut chunked_output);
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
