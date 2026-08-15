use super::candle_tasuki_gap::CandleTasukiGap;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleTasukiGap::default();
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
fn bulk_matches_scalar_at_every_split_and_after_continuation() {
    let len = 96;
    let close: Vec<_> = (0..len)
        .map(|i| 100.0 + (i as f64 * 0.31).sin() * 4.0)
        .collect();
    let open: Vec<_> = close
        .iter()
        .enumerate()
        .map(|(i, &c)| c + ((i * 17 % 13) as f64 - 6.0) * 0.19)
        .collect();
    let high: Vec<_> = open
        .iter()
        .zip(&close)
        .enumerate()
        .map(|(i, (&o, &c))| o.max(c) + 0.03 + (i % 7) as f64 * 0.11)
        .collect();
    let low: Vec<_> = open
        .iter()
        .zip(&close)
        .enumerate()
        .map(|(i, (&o, &c))| o.min(c) - 0.04 - (i % 5) as f64 * 0.13)
        .collect();
    let mut scalar = CandleTasukiGap::new();
    let expected: Vec<_> = (0..len)
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect();
    let expected_value = scalar.value();
    let expected_next = scalar.append(96.7, 97.2, 95.8, 96.1).unwrap_or(0);
    for split in 0..=len {
        let mut state = CandleTasukiGap::new();
        let mut actual = Vec::new();
        state
            .extend_slices_into(
                &open[..split],
                &high[..split],
                &low[..split],
                &close[..split],
                &mut actual,
            )
            .unwrap();
        state
            .extend_slices_into(
                &open[split..],
                &high[split..],
                &low[split..],
                &close[split..],
                &mut actual,
            )
            .unwrap();
        assert_eq!(actual, expected, "split {split}");
        assert_eq!(state.value(), expected_value, "value at split {split}");
        assert_eq!(
            state.append(96.7, 97.2, 95.8, 96.1).unwrap_or(0),
            expected_next,
            "continuation at split {split}"
        );
    }
}
