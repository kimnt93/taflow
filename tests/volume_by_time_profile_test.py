from tests.oracle_assertions import assert_registered_oracle_match


def test_volume_by_time_profile_matches_wickra() -> None:
    assert_registered_oracle_match("VolumeByTimeProfile")
