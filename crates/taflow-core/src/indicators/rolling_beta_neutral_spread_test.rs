use super::rolling_beta_neutral_spread::RollingBetaNeutralSpread;
#[test]
fn lifecycle() {
    let mut s = RollingBetaNeutralSpread::new(2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 4.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
