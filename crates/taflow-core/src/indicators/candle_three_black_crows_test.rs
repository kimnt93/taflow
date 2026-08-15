use super::candle_three_black_crows::CandleThreeBlackCrows;

fn inputs() -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut open = Vec::new();
    let mut high = Vec::new();
    let mut low = Vec::new();
    let mut close = Vec::new();
    for index in 0..97 {
        let center = 100.0 + (index as f64 * 0.31).sin() * 2.0 + index as f64 * 0.01;
        let o = center + (index as i32 % 5 - 2) as f64 * 0.13;
        let c = center + (index as i32 % 7 - 3) as f64 * 0.11;
        open.push(o);
        close.push(c);
        high.push(o.max(c) + 0.2 + (index % 3) as f64 * 0.02);
        low.push(o.min(c) - 0.2 - (index % 4) as f64 * 0.02);
    }
    (open, high, low, close)
}

#[test]
fn scalar_bulk_all_splits_continuation_and_reset_are_invariant() {
    let (open, high, low, close) = inputs();
    let mut scalar = CandleThreeBlackCrows::new();
    let expected: Vec<i32> = (0..open.len())
        .map(|index| {
            scalar
                .append(open[index], high[index], low[index], close[index])
                .unwrap_or(0)
        })
        .collect();
    let continuation = scalar.append(99.8, 100.4, 99.2, 99.4);

    for split in 0..=open.len() {
        let mut state = CandleThreeBlackCrows::new();
        let mut output = Vec::new();
        state
            .extend_slices_into(
                &open[..split],
                &high[..split],
                &low[..split],
                &close[..split],
                &mut output,
            )
            .unwrap();
        state
            .extend_slices_into(
                &open[split..],
                &high[split..],
                &low[split..],
                &close[split..],
                &mut output,
            )
            .unwrap();
        assert_eq!(output, expected);
        assert_eq!(state.value(), Some(*expected.last().unwrap()));
        assert_eq!(state.append(99.8, 100.4, 99.2, 99.4), continuation);
    }

    let mut state = CandleThreeBlackCrows::new();
    let mut output = Vec::new();
    state
        .extend_slices_into(&open, &high, &low, &close, &mut output)
        .unwrap();
    state.reset();
    assert_eq!(state.value(), None);
    output.clear();
    state
        .extend_slices_into(&open, &high, &low, &close, &mut output)
        .unwrap();
    assert_eq!(output, expected);
}

#[test]
fn misaligned_bulk_rejects_before_mutation() {
    let (open, high, low, close) = inputs();
    let mut state = CandleThreeBlackCrows::new();
    let mut output = vec![17];
    assert!(state
        .extend_slices_into(&open, &high, &low[..96], &close, &mut output)
        .is_err());
    assert_eq!(output, [17]);
    assert_eq!(state.value(), None);
}
