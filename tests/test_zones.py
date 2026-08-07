import pytest

from taflow import ActiveZoneList


def test_active_zone_list_is_bounded_and_mitigates():
    zones = ActiveZoneList(2)
    zones.add(10.0, 8.0)
    zones.add(6.0, 4.0)
    zones.add(3.0, 1.0)
    assert zones.size == 2
    assert zones.advance(5.0).tolist() == [True]
    assert zones.size == 1


def test_active_zone_list_rejects_zero_capacity():
    with pytest.raises(ValueError):
        ActiveZoneList(0)
