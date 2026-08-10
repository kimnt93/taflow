use super::rolling_kendall_rank_correlation::RollingKendallRankCorrelation;
#[test]
fn lifecycle() {
    let mut s = RollingKendallRankCorrelation::new(2).unwrap();
    s.append(1.0, 2.0);
    assert!(s.append(2.0, 3.0).is_some());
    s.reset();
    assert!(s.value().is_none());
}
