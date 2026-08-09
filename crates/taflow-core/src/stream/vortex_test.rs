use super::Vortex;

#[test]
fn vortex_replays_after_reset() {
    let mut state = Vortex::new(5).unwrap();
    let first: Vec<_> = (0..64)
        .map(|i| {
            let c = 100.0 + (i as f64 * 0.1).sin();
            state.append(c + 1.0, c - 1.0, c)
        })
        .collect();
    state.reset();
    let second: Vec<_> = (0..64)
        .map(|i| {
            let c = 100.0 + (i as f64 * 0.1).sin();
            state.append(c + 1.0, c - 1.0, c)
        })
        .collect();
    assert_eq!(
        first.iter().map(|v| v.vp.to_bits()).collect::<Vec<_>>(),
        second.iter().map(|v| v.vp.to_bits()).collect::<Vec<_>>()
    );
}
