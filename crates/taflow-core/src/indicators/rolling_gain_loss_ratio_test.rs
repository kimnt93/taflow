use super::rolling_gain_loss_ratio::RollingGainLossRatio;
#[test]
fn lifecycle() {
    let mut s = RollingGainLossRatio::new(2).unwrap();
    s.append(1.0);
    assert!(s.value().is_none());
    assert!(s.append(-1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
