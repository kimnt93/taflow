use super::ChaikinMoneyFlow;

#[test]
fn chaikin_money_flow_replays_after_reset() {
    let mut state = ChaikinMoneyFlow::new(5).unwrap();
    let first: Vec<_> = (0..64)
        .map(|i| {
            let c = 100.0 + i as f64;
            state.append(c + 1.0, c - 1.0, c, 1000.0)
        })
        .collect();
    state.reset();
    let second: Vec<_> = (0..64)
        .map(|i| {
            let c = 100.0 + i as f64;
            state.append(c + 1.0, c - 1.0, c, 1000.0)
        })
        .collect();
    assert_eq!(
        first
            .iter()
            .map(|v| v.map(f64::to_bits))
            .collect::<Vec<_>>(),
        second
            .iter()
            .map(|v| v.map(f64::to_bits))
            .collect::<Vec<_>>()
    );
}
