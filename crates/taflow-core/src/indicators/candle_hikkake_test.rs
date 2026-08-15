use super::candle_hikkake::CandleHikkake;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = CandleHikkake::new();
    for index in 0..16 {
        let price = 100.0 + index as f64;
        state.append(price, price + 2.0, price - 2.0, price + 0.5);
    }
    state.reset();
    assert_eq!(state.value(), None);
}

#[test]
fn bulk_chunks_and_pending_continuation_match_scalar() {
    let high = (0..257)
        .map(|index| 103.0 + ((index * 17) % 29) as f64 * 0.21)
        .collect::<Vec<_>>();
    let low = high
        .iter()
        .enumerate()
        .map(|(index, high)| high - 1.0 - (index % 5) as f64 * 0.17)
        .collect::<Vec<_>>();
    let close = high
        .iter()
        .zip(&low)
        .map(|(&high, &low)| (high + low) * 0.5)
        .collect::<Vec<_>>();
    let open = close.clone();
    let mut scalar = CandleHikkake::new();
    let expected = (0..open.len())
        .map(|i| {
            scalar
                .append(open[i], high[i], low[i], close[i])
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();
    let mut chunked = CandleHikkake::new();
    let mut actual = Vec::new();
    chunked
        .extend_slices_into(&open[..1], &high[..1], &low[..1], &close[..1], &mut actual)
        .unwrap();
    chunked
        .extend_slices_into(&open[1..], &high[1..], &low[1..], &close[1..], &mut actual)
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(chunked.value(), scalar.value());
    assert_eq!(
        chunked.append(105.0, 106.0, 103.0, 105.5),
        scalar.append(105.0, 106.0, 103.0, 105.5)
    );
}
