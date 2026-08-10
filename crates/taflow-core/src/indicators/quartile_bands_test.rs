use super::quartile_bands::QuartileBands;
#[test]
fn lifecycle() {
    let mut s = QuartileBands::new(3).unwrap();
    s.append(1.);
    s.append(2.);
    assert!(s.append(3.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
