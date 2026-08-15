use super::acceleration_bands::AccelerationBands;

fn value_bits(
    value: Option<super::acceleration_bands::AccelerationBandsValue>,
) -> Option<[u64; 3]> {
    value.map(|value| {
        [
            value.upper.to_bits(),
            value.middle.to_bits(),
            value.lower.to_bits(),
        ]
    })
}

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = AccelerationBands::new(3).unwrap();
    for index in 0..8 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    let value = state.value();
    state.reset();
    assert_eq!(state.value(), None);
    for index in 0..8 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    assert_eq!(state.value(), value);
}

#[test]
fn bulk_all_splits_and_continuation_are_bitwise_identical() {
    let close: Vec<_> = (0..257)
        .map(|index| 100.0 + index as f64 * 0.03 + ((index * 37) % 29) as f64 * 0.07)
        .collect();
    let high: Vec<_> = close
        .iter()
        .enumerate()
        .map(|(index, &close)| close + 0.4 + (index % 5) as f64 * 0.03)
        .collect();
    let low: Vec<_> = close
        .iter()
        .enumerate()
        .map(|(index, &close)| close - 0.5 - (index % 7) as f64 * 0.02)
        .collect();

    for period in [2, 3, 20, 30] {
        let mut scalar = AccelerationBands::new(period).unwrap();
        let expected: Vec<_> = (0..close.len())
            .map(|index| value_bits(scalar.append(high[index], low[index], close[index])))
            .collect();
        let continuation = value_bits(scalar.append(110.0, 108.0, 109.0));

        for split in 0..=close.len() {
            let mut state = AccelerationBands::new(period).unwrap();
            let (mut upper, mut middle, mut lower) = (Vec::new(), Vec::new(), Vec::new());
            state
                .extend_slices_into(
                    &high[..split],
                    &low[..split],
                    &close[..split],
                    &mut upper,
                    &mut middle,
                    &mut lower,
                )
                .unwrap();
            state
                .extend_slices_into(
                    &high[split..],
                    &low[split..],
                    &close[split..],
                    &mut upper,
                    &mut middle,
                    &mut lower,
                )
                .unwrap();
            let actual: Vec<_> = upper
                .iter()
                .zip(&middle)
                .zip(&lower)
                .map(|((&upper, &middle), &lower)| {
                    if upper.is_nan() {
                        None
                    } else {
                        Some([upper.to_bits(), middle.to_bits(), lower.to_bits()])
                    }
                })
                .collect();
            assert_eq!(actual, expected);
            assert_eq!(value_bits(state.append(110.0, 108.0, 109.0)), continuation);
        }
    }
}

#[test]
fn bulk_validation_does_not_mutate_outputs() {
    let mut state = AccelerationBands::new(3).unwrap();
    let (mut upper, mut middle, mut lower) = (vec![1.0], vec![2.0], vec![3.0]);
    assert!(state
        .extend_slices_into(&[101.0], &[], &[100.0], &mut upper, &mut middle, &mut lower,)
        .is_err());
    assert_eq!(upper, [1.0]);
    assert_eq!(middle, [2.0]);
    assert_eq!(lower, [3.0]);
    assert_eq!(state.value(), None);
}
