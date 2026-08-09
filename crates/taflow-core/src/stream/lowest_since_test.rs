use super::lowest_since::LowestSince;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = LowestSince::new();
    assert_eq!(state.append(false, 3.0), Some(3.0));
    assert_eq!(state.append(true, 2.0), Some(2.0));
    assert_eq!(state.append(false, 4.0), Some(2.0));
    state.reset();
    assert_eq!(state.value(), None);
}
