use super::hurst_channel::HurstChannel;
#[test]
fn lifecycle() {
    let mut s = HurstChannel::new(2, 0.5).unwrap();
    s.append(2.0, 1.0, 1.5);
    assert!(s.append(3.0, 2.0, 2.5).is_some());
    s.reset();
    assert!(s.value().is_none());
}
