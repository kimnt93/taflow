use super::Supertrend;

#[test]
fn supertrend_warmup_and_reset_are_deterministic() {
    let high: Vec<f64> = (0..96).map(|i| 100.0 + (i as f64 * 0.1).sin()).collect();
    let low: Vec<f64> = high.iter().map(|v| v - 2.0).collect();
    let close: Vec<f64> = high.iter().map(|v| v - 1.0).collect();
    let mut state = Supertrend::new(7, 3.0).unwrap();
    let first: Vec<f64> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| state.append(h, l, c).map_or(f64::NAN, |v| v.trend))
        .collect();
    assert!(first[..6].iter().all(|v| v.is_nan()));
    assert!(first[6..].iter().all(|v| v.is_finite()));
    state.reset();
    let second: Vec<f64> = high
        .iter()
        .zip(&low)
        .zip(&close)
        .map(|((&h, &l), &c)| state.append(h, l, c).map_or(f64::NAN, |v| v.trend))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
}
