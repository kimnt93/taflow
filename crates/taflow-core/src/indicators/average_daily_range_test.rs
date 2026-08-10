use super::average_daily_range::AverageDailyRange;
#[test]
fn lifecycle() {
    let mut s = AverageDailyRange::new(2, 0).unwrap();
    s.append(1.0, 3.0, 1.0, 2.0, 1.0, 0);
    assert!(s
        .append(2.0, 4.0, 2.0, 3.0, 1.0, 86_400_000_000_000)
        .is_some());
    s.reset();
    assert!(s.value().is_none());
}
