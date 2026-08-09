use super::TimeSeriesRank;
use crate::stream::StreamingIndicator;

#[test]
fn time_series_rank_replays_after_reset() {
    let mut state = TimeSeriesRank::new(4).unwrap();
    let first: Vec<_> = (1..=16).map(|v| state.append(v as f64)).collect();
    state.reset();
    let second: Vec<_> = (1..=16).map(|v| state.append(v as f64)).collect();
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
