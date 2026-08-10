use super::new_highs_new_lows::NewHighsNewLows;
#[test]
fn lifecycle() {
    let mut s = NewHighsNewLows::new().unwrap();
    assert_eq!(s.append(3., 1.), Some(2.));
    s.reset();
    assert!(s.value().is_none());
}
