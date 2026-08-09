use super::garman_klass_yang_zhang::GarmanKlassYangZhang;

#[test]
fn reset_replays_identically() {
    let mut state = GarmanKlassYangZhang::new(10).unwrap();
    let first: Vec<u64> = (0..100)
        .map(|index| {
            state
                .append(
                    100.0 + index as f64,
                    101.0 + index as f64,
                    99.0 + index as f64,
                    100.5 + index as f64,
                )
                .unwrap_or(f64::NAN)
                .to_bits()
        })
        .collect();
    state.reset();
    let second: Vec<u64> = (0..100)
        .map(|index| {
            state
                .append(
                    100.0 + index as f64,
                    101.0 + index as f64,
                    99.0 + index as f64,
                    100.5 + index as f64,
                )
                .unwrap_or(f64::NAN)
                .to_bits()
        })
        .collect();
    assert_eq!(first, second);
}
