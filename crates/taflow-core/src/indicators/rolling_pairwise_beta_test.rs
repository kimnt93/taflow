use super::rolling_pairwise_beta::RollingPairwiseBeta;
#[test]
fn lifecycle() {
    let mut s = RollingPairwiseBeta::new(2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 4.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
