use super::linear_regression_channel::LinearRegressionChannel;
#[test]
fn lifecycle() {
    let mut s = LinearRegressionChannel::new(2).unwrap();
    s.append(1.0);
    assert!(s.append(2.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
