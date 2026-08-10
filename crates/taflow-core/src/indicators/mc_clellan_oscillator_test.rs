use super::mc_clellan_oscillator::McClellanOscillator;
#[test]
fn lifecycle() {
    let mut s = McClellanOscillator::new().unwrap();
    assert!(s.append(1., 0., 0., 0.).is_some());
    s.reset();
    assert!(s.value().is_none());
}
