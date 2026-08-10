from oracle_assertions import assert_registered_oracle_match


def test_roofing_filter_matches_wickra() -> None:
    assert_registered_oracle_match("RoofingFilter")
