use super::KnowSureThing;

#[test]
fn know_sure_thing_replays_after_reset() {
    let mut state = KnowSureThing::new(3, 4, 5, 6, 3, 3, 3, 3, 3).unwrap();
    let first: Vec<_> = (0..128)
        .map(|i| state.append(100.0 + i as f64 * 0.2))
        .collect();
    state.reset();
    let second: Vec<_> = (0..128)
        .map(|i| state.append(100.0 + i as f64 * 0.2))
        .collect();
    assert_eq!(
        first.iter().map(|v| v.kst.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.kst.to_bits()).collect::<Vec<_>>()
    );
}
