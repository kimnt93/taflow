from tests.oracle_assertions import assert_registered_oracle_match


def test_high_low_index_matches_wickra() -> None:
    assert_registered_oracle_match("HighLowIndex")
