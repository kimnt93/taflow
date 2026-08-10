use super::intraday_volatility_profile::IntradayVolatilityProfile;
#[test]
fn lifecycle() {
    let mut s = IntradayVolatilityProfile::new(24, 0).unwrap();
    s.append(1., 1., 1., 1., 1., 0);
    assert!(s.append(1., 2., 1., 2., 1., 1).is_some());
    s.reset();
    assert!(s.value().is_none());
}
