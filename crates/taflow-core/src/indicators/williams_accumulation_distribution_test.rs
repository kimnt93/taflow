use super::williams_accumulation_distribution::WilliamsAccumulationDistribution;
#[test]
fn lifecycle() {
    let mut s = WilliamsAccumulationDistribution::new().unwrap();
    assert!(s.append(2.0, 1.0, 1.5).is_none());
    assert!(s.append(3.0, 1.5, 2.5).is_some());
    s.reset();
    assert!(s.value().is_none());
}
