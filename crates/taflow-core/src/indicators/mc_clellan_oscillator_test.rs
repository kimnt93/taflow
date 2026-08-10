use super::mc_clellan_oscillator::McClellanOscillator;
#[test]
fn lifecycle() {
    let mut s = McClellanOscillator::new().unwrap();
    assert_eq!(s.append(3., 1.), Some(0.0));
    s.reset();
    assert!(s.value().is_none());
}
