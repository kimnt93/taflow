use super::flag_pennant::FlagPennant;
#[test]
fn lifecycle() {
    let mut s = FlagPennant::new().unwrap();
    for i in 0..20 {
        s.append(1., 3., 0., i as f64);
    }
    assert!(s.value().is_some());
    s.reset();
    assert!(s.value().is_none());
}
