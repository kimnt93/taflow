use super::percent_above_moving_average::PercentAboveMovingAverage;
#[test]
fn lifecycle() {
    let mut s = PercentAboveMovingAverage::new().unwrap();
    assert_eq!(s.append(2., 5.), Some(40.));
    s.reset();
    assert!(s.value().is_none());
}
