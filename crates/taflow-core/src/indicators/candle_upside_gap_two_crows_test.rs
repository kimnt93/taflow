use super::candle_upside_gap_two_crows::CandleUpsideGapTwoCrows;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleUpsideGapTwoCrows::default();
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
fn bulk_chunk_and_continuation_match_scalar() {
    let open = (0..257)
        .map(|index| 100.0 + ((index * 17) % 31) as f64 * 0.23)
        .collect::<Vec<_>>();
    let close = open
        .iter()
        .enumerate()
        .map(|(index, open)| open + ((index * 13) % 11) as f64 * 0.31 - 1.55)
        .collect::<Vec<_>>();
    let high = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.max(c) + 0.8)
        .collect::<Vec<_>>();
    let low = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.min(c) - 0.7)
        .collect::<Vec<_>>();
    let mut scalar = CandleUpsideGapTwoCrows::new();
    let expected = (0..open.len())
        .map(|index| {
            scalar
                .append(open[index], high[index], low[index], close[index])
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    let mut chunked = CandleUpsideGapTwoCrows::new();
    let mut actual = Vec::new();
    chunked
        .extend_slices_into(
            &open[..10],
            &high[..10],
            &low[..10],
            &close[..10],
            &mut actual,
        )
        .unwrap();
    chunked
        .extend_slices_into(
            &open[10..],
            &high[10..],
            &low[10..],
            &close[10..],
            &mut actual,
        )
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(chunked.value(), scalar.value());
    assert_eq!(
        chunked.append(106.0, 108.0, 103.0, 104.0),
        scalar.append(106.0, 108.0, 103.0, 104.0)
    );
}
