from oracle_assertions import assert_registered_oracle_match


def test_median_channel_matches_wickra() -> None:
    assert_registered_oracle_match("MedianChannel")
