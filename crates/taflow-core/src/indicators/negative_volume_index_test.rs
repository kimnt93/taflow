use super::NegativeVolumeIndex;

#[test]
fn negative_volume_index_replays_after_reset() {
    let mut state = NegativeVolumeIndex::new();
    let first: Vec<_> = (0..32)
        .map(|i| state.append(100.0 + i as f64, 1000.0 + (i % 3) as f64))
        .collect();
    state.reset();
    let second: Vec<_> = (0..32)
        .map(|i| state.append(100.0 + i as f64, 1000.0 + (i % 3) as f64))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
    );
}
