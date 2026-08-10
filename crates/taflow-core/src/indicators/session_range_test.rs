use super::session_range::SessionRange;
#[test]
fn lifecycle() {
    let mut s = SessionRange::new(0).unwrap();
    let value = s.append(1.0, 3.0, 1.0, 2.0, 1.0, 0).unwrap();
    assert_eq!(
        (value.asia, value.europe, value.united_states),
        (2.0, 0.0, 0.0)
    );
    s.reset();
    assert!(s.value().is_none());
}
