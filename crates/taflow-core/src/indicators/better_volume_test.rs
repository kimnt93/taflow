use super::better_volume::BetterVolume;
#[test]
fn lifecycle() {
    let mut s = BetterVolume::new(2).unwrap();
    assert!(s.append(2.0, 1.0, 1.5, 10.0).is_none());
    assert!(s.append(3.0, 1.0, 2.0, 20.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
