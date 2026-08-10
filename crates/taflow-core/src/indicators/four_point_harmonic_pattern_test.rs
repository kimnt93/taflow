use super::four_point_harmonic_pattern::FourPointHarmonicPattern;

#[test]
fn lifecycle_is_causal_and_resettable() {
    let mut state = FourPointHarmonicPattern::new().unwrap();
    for index in 0..5 {
        state.append(
            10.0,
            12.0 + index as f64,
            8.0 - index as f64,
            10.0 + index as f64,
        );
    }
    assert_eq!(state.len(), 5);
    assert!(state.value().is_some());
    state.reset();
    assert!(state.is_empty());
    assert!(state.value().is_none());
}
