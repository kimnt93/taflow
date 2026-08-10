use super::fractal_dimension::FractalDimension;

#[test]
fn warmup_and_reset_are_consistent() {
    let mut state = FractalDimension::new(4).unwrap();
    assert_eq!(state.append(1.0), None);
    assert_eq!(state.append(2.0), None);
    assert_eq!(state.append(3.0), None);
    assert!(state.append(4.0).is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
