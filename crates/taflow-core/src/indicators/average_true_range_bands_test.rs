use super::average_true_range_bands::AverageTrueRangeBands;
#[test]
fn lifecycle() {
    let mut s = AverageTrueRangeBands::new(2, 2.0).unwrap();
    s.append(2.0, 1.0, 1.5);
    assert!(s.append(2.0, 1.0, 1.5).is_none());
    assert!(s.append(2.0, 1.0, 1.5).is_some());
    s.reset();
    assert!(s.value().is_none());
}
