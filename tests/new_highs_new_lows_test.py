from tests.oracle_assertions import assert_registered_oracle_match


def test_new_highs_new_lows_matches_wickra() -> None:
    assert_registered_oracle_match("NewHighsNewLows")
