from oracle_assertions import assert_registered_oracle_match


def test_instantaneous_trendline_matches_wickra() -> None:
    assert_registered_oracle_match("InstantaneousTrendline")
