use super::OrderBlock;

#[test]
fn order_block_resets() {
    let bars: Vec<(f64, f64, f64, f64)> = (0..128)
        .map(|i| {
            let close = 100.0 + (i as f64 * 0.13).sin() * 3.0;
            (close + 1.0, close - 1.0, close, 1_000.0 + i as f64)
        })
        .collect();
    let mut state = OrderBlock::new(5, 3, 14, 2.0).unwrap();
    let first: Vec<_> = bars
        .iter()
        .map(|&(h, l, c, v)| state.append(h, l, c, v))
        .collect();
    state.reset();
    let second: Vec<_> = bars
        .iter()
        .map(|&(h, l, c, v)| state.append(h, l, c, v))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.ob.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.ob.to_bits()).collect::<Vec<_>>()
    );
}
