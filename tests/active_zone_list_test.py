from taflow import ActiveZoneList


def test_active_zone_list_lifecycle():
    zones = ActiveZoneList(capacity=2)
    zones.add(2.0, 1.0)
    assert len(zones) == 1
    zones.advance(1.5)
    assert len(zones) == 0
    assert zones.reset() is zones
