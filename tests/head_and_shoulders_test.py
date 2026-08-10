from tests.oracle_assertions import assert_registered_oracle_match


def test_head_and_shoulders_matches_wickra() -> None:
    assert_registered_oracle_match("HeadAndShoulders")
