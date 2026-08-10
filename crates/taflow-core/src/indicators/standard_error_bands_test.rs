use super::standard_error_bands::StandardErrorBands;
#[test]
fn lifecycle() {
    let mut s = StandardErrorBands::new(3, 2.0).unwrap();
    s.append(1.0);
    s.append(2.0);
    assert!(s.append(3.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
