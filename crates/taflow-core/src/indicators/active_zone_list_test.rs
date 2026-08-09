use super::active_zone_list::ActiveZoneList;

#[test]
fn bounded_zones_are_advanced_and_reset() {
    let mut zones = ActiveZoneList::new(2).unwrap();
    zones.add(2.0, 1.0, 0);
    assert_eq!(zones.zone_count(), 1);
    assert_eq!(zones.advance(1.5, None), Vec::<bool>::new());
    assert_eq!(zones.zone_count(), 0);
    zones.reset();
    assert_eq!(zones.zone_count(), 0);
}
