use super::instantaneous_trendline::InstantaneousTrendline;
#[test]
fn lifecycle() {
    let mut s = InstantaneousTrendline::new(3).unwrap();
    assert!(s.append(1.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
