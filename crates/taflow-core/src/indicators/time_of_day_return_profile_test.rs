use super::time_of_day_return_profile::TimeOfDayReturnProfile;
#[test]
fn lifecycle() {
    let mut s = TimeOfDayReturnProfile::new(24, 0).unwrap();
    s.append(1.0, 1.0, 1.0, 1.0, 1.0, 0);
    assert!(s.append(1.0, 2.0, 1.0, 2.0, 1.0, 1).is_some());
    s.reset();
    assert!(s.value().is_none());
}
