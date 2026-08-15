use super::candle_three_white_soldiers::CandleThreeWhiteSoldiers;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleThreeWhiteSoldiers::default();
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
fn bulk_all_splits_and_continuation_match_scalar_replay() {
    let open: Vec<_> = (0..96).map(|i| 100.0 + i as f64 * 0.07).collect();
    let close: Vec<_> = open
        .iter()
        .enumerate()
        .map(|(i, &value)| value + (i % 9) as f64 * 0.05 - 0.2)
        .collect();
    let high: Vec<_> = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.max(c) + 0.6)
        .collect();
    let low: Vec<_> = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.min(c) - 0.5)
        .collect();
    let mut scalar = CandleThreeWhiteSoldiers::new();
    let expected: Vec<_> = (0..open.len())
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect();
    let continuation = scalar.append(108.0, 109.0, 107.0, 108.5);
    for split in 0..=open.len() {
        let mut state = CandleThreeWhiteSoldiers::new();
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
        assert_eq!(actual, expected);
        assert_eq!(state.append(108.0, 109.0, 107.0, 108.5), continuation);
    }
}
