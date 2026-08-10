use super::overnight_intraday_return::OvernightIntradayReturn;
#[test]
fn lifecycle() {
    let mut s = OvernightIntradayReturn::new(0).unwrap();
    s.append(1.0, 1.0, 1.0, 2.0, 1.0, 0);
    assert!(s
        .append(2.0, 3.0, 2.0, 3.0, 1.0, 86_400_000_000_000)
        .is_some());
    s.reset();
    assert!(s.value().is_none());
}
