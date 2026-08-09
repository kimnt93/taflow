use super::inside_bar::InsideBar;

#[test]
fn lifecycle_and_reset_are_causal() {
    let mut state = InsideBar::new();
    assert_eq!(state.append(10.0, 8.0), None);
    assert_eq!(state.append(9.0, 8.5), Some(1.0));
    assert_eq!(state.value(), Some(1.0));
    state.reset();
    assert_eq!(state.value(), None);
}
