use super::VolumePriceTrend;

#[test]
fn volume_price_trend_replays_after_reset() {
    let mut state = VolumePriceTrend::new();
    let first: Vec<_> = (0..32)
        .map(|i| state.append(100.0 + i as f64, 1000.0))
        .collect();
    state.reset();
    let second: Vec<_> = (0..32)
        .map(|i| state.append(100.0 + i as f64, 1000.0))
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
