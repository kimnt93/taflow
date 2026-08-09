use super::McGinleyDynamic;

#[test]
fn mcginley_dynamic_replays_after_reset() {
    let mut state = McGinleyDynamic::new(10, 0.6).unwrap();
    let first: Vec<_> = (0..64).map(|i| state.append(100.0 + i as f64)).collect();
    state.reset();
    let second: Vec<_> = (0..64).map(|i| state.append(100.0 + i as f64)).collect();
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
