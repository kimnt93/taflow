use super::Ichimoku;

#[test]
fn ichimoku_outputs_are_causal_and_replayable() {
    let high: Vec<f64> = (0..80).map(|i| 50.0 + i as f64).collect();
    let low: Vec<f64> = high.iter().map(|v| v - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|v| v - 1.0).collect();
    let mut state = Ichimoku::new(9, 26, 52).unwrap();
    let first: Vec<_> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| state.append(h, l, c))
        .collect();
    assert!(first[0].tenkan_sen.is_nan());
    assert_eq!(first[30].chikou_span, close[30]);
    state.reset();
    let second: Vec<_> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| state.append(h, l, c))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.span_b.to_bits()).collect::<Vec<_>>(),
        second
            .iter()
            .map(|v| v.span_b.to_bits())
            .collect::<Vec<_>>()
    );
}
