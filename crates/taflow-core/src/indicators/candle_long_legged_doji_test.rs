use super::candle_long_legged_doji::CandleLongLeggedDoji;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleLongLeggedDoji::default();
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
fn bulk_is_bitwise_identical_for_every_chunk_split_and_continuation() {
    let len = 96;
    let close: Vec<_> = (0..len)
        .map(|i| 100.0 + (i as f64 * 0.31).sin() * 4.0 + i as f64 * 0.02)
        .collect();
    let open: Vec<_> = close
        .iter()
        .enumerate()
        .map(|(i, &close)| close + ((i * 17 % 13) as f64 - 6.0) * 0.19)
        .collect();
    let high: Vec<_> = open
        .iter()
        .zip(&close)
        .enumerate()
        .map(|(i, (&open, &close))| open.max(close) + 0.03 + (i % 7) as f64 * 0.11)
        .collect();
    let low: Vec<_> = open
        .iter()
        .zip(&close)
        .enumerate()
        .map(|(i, (&open, &close))| open.min(close) - 0.04 - (i % 5) as f64 * 0.13)
        .collect();

    let mut scalar = CandleLongLeggedDoji::new();
    let expected: Vec<_> = (0..len)
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect();
    let expected_value = scalar.value();
    let continuation = (96.7, 97.2, 95.8, 96.71);
    let expected_continuation = scalar
        .append(
            continuation.0,
            continuation.1,
            continuation.2,
            continuation.3,
        )
        .unwrap_or(0);

    for split in 0..=len {
        let mut chunked = CandleLongLeggedDoji::new();
        let mut actual = Vec::new();
        chunked
            .extend_slices_into(
                &open[..split],
                &high[..split],
                &low[..split],
                &close[..split],
                &mut actual,
            )
            .unwrap();
        chunked
            .extend_slices_into(
                &open[split..],
                &high[split..],
                &low[split..],
                &close[split..],
                &mut actual,
            )
            .unwrap();
        assert_eq!(actual, expected, "split {split}");
        assert_eq!(chunked.value(), expected_value, "value at split {split}");
        assert_eq!(
            chunked
                .append(
                    continuation.0,
                    continuation.1,
                    continuation.2,
                    continuation.3,
                )
                .unwrap_or(0),
            expected_continuation,
            "continuation at split {split}"
        );
    }
}
