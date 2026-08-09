use super::williams_percent_r::WilliamsPercentR;

fn generated_series(length: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let close: Vec<_> = (0..length)
        .map(|index| 80.0 + index as f64 * 0.03 + (index as f64 * 0.19).sin() * 4.0)
        .collect();
    let high = close.iter().map(|value| value + 1.25).collect();
    let low = close.iter().map(|value| value - 1.25).collect();
    (high, low, close)
}

#[test]
fn scalar_bulk_reset_and_continuation_are_bitwise_identical() {
    let generated = generated_series(5_000);
    let flat = (
        vec![7.5_f64; 5_000],
        vec![7.5_f64; 5_000],
        vec![7.5_f64; 5_000],
    );
    for (high, low, close) in [generated, flat] {
        for period in [2_usize, 5, 14, 30, 200] {
            let mut scalar_state = WilliamsPercentR::new(period).unwrap();
            let scalar: Vec<_> = (0..close.len())
                .map(|index| {
                    scalar_state
                        .append(high[index], low[index], close[index])
                        .unwrap_or(f64::NAN)
                })
                .collect();
            assert!(scalar[..period - 1].iter().all(|value| value.is_nan()));

            let mut bulk_state = WilliamsPercentR::new(period).unwrap();
            let mut bulk = Vec::new();
            for chunk in [1_usize, 7, 97, close.len()] {
                bulk.clear();
                bulk_state.reset();
                let mut offset = 0;
                while offset < close.len() {
                    let end = (offset + chunk).min(close.len());
                    bulk_state
                        .extend_slices_into(
                            &high[offset..end],
                            &low[offset..end],
                            &close[offset..end],
                            &mut bulk,
                        )
                        .unwrap();
                    offset = end;
                }
                for (actual, expected) in bulk.iter().zip(&scalar) {
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "period={period} chunk={chunk}"
                    );
                }
                assert_eq!(bulk_state.value(), scalar_state.value());
                assert_eq!(
                    bulk_state.append(11.0, 9.0, 10.0),
                    scalar_state.clone().append(11.0, 9.0, 10.0)
                );
            }
        }
    }
}

#[test]
fn configuration_lengths_and_short_input_are_consistent() {
    assert!(WilliamsPercentR::new(1).is_err());
    let mut state = WilliamsPercentR::new(4).unwrap();
    let mut output = Vec::new();
    state
        .extend_slices_into(&[5.0, 5.0], &[5.0, 5.0], &[5.0, 5.0], &mut output)
        .unwrap();
    assert!(output.iter().all(|value| value.is_nan()));
    assert_eq!(state.value(), None);
    let before = output.clone();
    assert!(state
        .extend_slices_into(&[1.0, 2.0], &[1.0], &[1.0, 2.0], &mut output)
        .is_err());
    assert_eq!(output.len(), before.len());
}
