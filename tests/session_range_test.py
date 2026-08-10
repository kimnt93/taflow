from oracle_assertions import assert_registered_oracle_match


def test_session_range_matches_wickra() -> None:
    assert_registered_oracle_match("SessionRange")
