use super::candle_high_wave::CandleHighWave;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleHighWave::default();
    for index in 0..12 {
        state.append(
            100.0 + index as f64,
            102.0 + index as f64,
            98.0 + index as f64,
            101.0 + index as f64,
        );
    }
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_chunks_and_continuation_match_scalar() {
    let open: Vec<f64> = (0..137)
        .map(|i| 90.0 + (i as f64 * 0.17).sin() * 2.0)
        .collect();
    let close: Vec<f64> = open
        .iter()
        .enumerate()
        .map(|(i, &value)| value + (i as f64 * 0.31).cos() * 0.7)
        .collect();
    let high: Vec<f64> = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.max(c) + 1.1)
        .collect();
    let low: Vec<f64> = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.min(c) - 1.2)
        .collect();
    let mut scalar = CandleHighWave::new();
    let expected: Vec<_> = (0..open.len())
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect();
    let expected_value = scalar.value();
    let expected_next = scalar.append(91.0, 93.0, 89.0, 90.5);
    for size in [1, 11, open.len()] {
        let mut bulk = CandleHighWave::new();
        let mut actual = Vec::new();
        for start in (0..open.len()).step_by(size) {
            let end = (start + size).min(open.len());
            bulk.extend_slices_into(
                &open[start..end],
                &high[start..end],
                &low[start..end],
                &close[start..end],
                &mut actual,
            )
            .unwrap();
        }
        assert_eq!(actual, expected);
        assert_eq!(bulk.value(), expected_value);
        assert_eq!(bulk.append(91.0, 93.0, 89.0, 90.5), expected_next);
    }
}
