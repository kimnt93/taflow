use super::decycler_oscillator::DecyclerOscillator;
#[test]
fn lifecycle() {
    let mut s = DecyclerOscillator::new(2, 4).unwrap();
    assert!(s.append(1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
