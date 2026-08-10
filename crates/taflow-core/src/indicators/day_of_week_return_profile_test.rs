use super::day_of_week_return_profile::DayOfWeekReturnProfile;
#[test]
fn lifecycle() {
    let mut s = DayOfWeekReturnProfile::new(0).unwrap();
    s.append(1.0, 1.0, 1.0, 1.0, 1.0, 0);
    assert!(s.append(1.0, 2.0, 1.0, 2.0, 1.0, 1).is_some());
    s.reset();
    assert!(s.value().is_none());
}
