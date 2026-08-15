use super::candle_stalled_pattern::CandleStalledPattern;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleStalledPattern::default();
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
fn bulk_chunks_continuation_and_validation_match_scalar_replay() {
    let open: Vec<_> = (0..80)
        .map(|index| 100.0 + (index as f64 * 0.31).sin() * 3.0)
        .collect();
    let high: Vec<_> = open
        .iter()
        .enumerate()
        .map(|(index, &value)| value + 1.0 + (index % 4) as f64 * 0.2)
        .collect();
    let low: Vec<_> = open
        .iter()
        .enumerate()
        .map(|(index, &value)| value - 1.1 - (index % 3) as f64 * 0.15)
        .collect();
    let close: Vec<_> = open
        .iter()
        .enumerate()
        .map(|(index, &value)| value + ((index % 7) as f64 - 3.0) * 0.22)
        .collect();

    let mut scalar = CandleStalledPattern::default();
    let expected: Vec<_> = (0..open.len())
        .map(|index| {
            scalar
                .append(open[index], high[index], low[index], close[index])
                .unwrap_or(0)
        })
        .collect();
    let continuation = scalar.append(102.0, 103.5, 100.5, 102.7);

    for split in 0..=open.len() {
        let mut chunked = CandleStalledPattern::default();
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
        assert_eq!(
            chunked.append(102.0, 103.5, 100.5, 102.7),
            continuation,
            "continuation split {split}"
        );
    }

    let mut state = CandleStalledPattern::default();
    let mut output = vec![17];
    assert!(state
        .extend_slices_into(&open[..3], &high[..2], &low[..3], &close[..3], &mut output)
        .is_err());
    assert_eq!(output, vec![17]);
    assert_eq!(state.value(), None);
}
