use super::candle_rise_fall_three_methods::CandleRiseFallThreeMethods;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleRiseFallThreeMethods::default();
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
        .map(|index| 100.0 + (index as f64 * 0.27).sin() * 4.0)
        .collect();
    let high: Vec<_> = open.iter().map(|&value| value + 1.8).collect();
    let low: Vec<_> = open.iter().map(|&value| value - 1.7).collect();
    let close: Vec<_> = open
        .iter()
        .enumerate()
        .map(|(index, &value)| value + ((index % 9) as f64 - 4.0) * 0.19)
        .collect();
    let mut scalar = CandleRiseFallThreeMethods::default();
    let expected: Vec<_> = (0..open.len())
        .map(|index| {
            scalar
                .append(open[index], high[index], low[index], close[index])
                .unwrap_or(0)
        })
        .collect();
    let continuation = scalar.append(104.0, 106.0, 102.0, 105.0);

    for split in 0..=open.len() {
        let mut chunked = CandleRiseFallThreeMethods::default();
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
            chunked.append(104.0, 106.0, 102.0, 105.0),
            continuation,
            "continuation split {split}"
        );
    }

    let mut state = CandleRiseFallThreeMethods::default();
    let mut output = vec![23];
    assert!(state
        .extend_slices_into(&open[..3], &high[..3], &low[..2], &close[..3], &mut output)
        .is_err());
    assert_eq!(output, vec![23]);
    assert_eq!(state.value(), None);
}
