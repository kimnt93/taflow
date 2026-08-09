use super::Liquidity;

#[test]
fn liquidity_resets() {
    let bars: Vec<(f64, f64)> = (0..128)
        .map(|i| {
            let close = 100.0 + (i as f64 * 0.19).sin() * 4.0;
            (close + 1.0, close - 1.0)
        })
        .collect();
    let mut state = Liquidity::new(3, 0.01).unwrap();
    let first: Vec<_> = bars
        .iter()
        .map(|&(h, l)| state.append(h, l, f64::NAN))
        .collect();
    state.reset();
    let second: Vec<_> = bars
        .iter()
        .map(|&(h, l)| state.append(h, l, f64::NAN))
        .collect();
    assert_eq!(
        first
            .iter()
            .map(|v| v.liquidity.to_bits())
            .collect::<Vec<_>>(),
        second
            .iter()
            .map(|v| v.liquidity.to_bits())
            .collect::<Vec<_>>()
    );
}
