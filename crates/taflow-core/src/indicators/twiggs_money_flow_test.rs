use super::twiggs_money_flow::TwiggsMoneyFlow;
#[test]
fn lifecycle() {
    let mut s = TwiggsMoneyFlow::new(2).unwrap();
    s.append(2.0, 1.0, 1.5, 10.0);
    assert!(s.append(2.0, 1.0, 1.5, 10.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
