use super::candle_kicking_by_length::CandleKickingByLength;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleKickingByLength::default();
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
    let open = (0..257)
        .map(|i| 100.0 + ((i * 17) % 31) as f64 * 0.23)
        .collect::<Vec<_>>();
    let close = open
        .iter()
        .enumerate()
        .map(|(i, o)| o + ((i * 13) % 11) as f64 * 0.31 - 1.55)
        .collect::<Vec<_>>();
    let high = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.max(c) + 0.08)
        .collect::<Vec<_>>();
    let low = open
        .iter()
        .zip(&close)
        .map(|(&o, &c)| o.min(c) - 0.07)
        .collect::<Vec<_>>();
    let mut scalar = CandleKickingByLength::new();
    let expected = (0..open.len())
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    let mut chunked = CandleKickingByLength::new();
    let mut actual = Vec::new();
    chunked
        .extend_slices_into(&open[..7], &high[..7], &low[..7], &close[..7], &mut actual)
        .unwrap();
    chunked
        .extend_slices_into(&open[7..], &high[7..], &low[7..], &close[7..], &mut actual)
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(chunked.value(), scalar.value());
    assert_eq!(
        chunked.append(105.0, 108.0, 102.0, 106.0),
        scalar.append(105.0, 108.0, 102.0, 106.0)
    );
}
