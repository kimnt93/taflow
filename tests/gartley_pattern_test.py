from tests.oracle_assertions import assert_registered_oracle_match


def test_gartley_pattern_matches_wickra() -> None:
    assert_registered_oracle_match("GartleyPattern")
