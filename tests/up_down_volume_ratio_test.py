from tests.oracle_assertions import assert_registered_oracle_match


def test_up_down_volume_ratio_matches_wickra() -> None:
    assert_registered_oracle_match("UpDownVolumeRatio")
