use super::average_directional_index_rating::AverageDirectionalIndexRating;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = AverageDirectionalIndexRating::new(14).unwrap();
    for index in 0..50 {
        state.append(
            101.0 + index as f64,
            99.0 + index as f64,
            100.0 + index as f64,
        );
    }
    assert!(state.value().is_some());
    state.reset();
    assert_eq!(state.value(), None);
}
