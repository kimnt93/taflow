from tests.oracle_assertions import assert_registered_oracle_match


def test_triple_top_bottom_matches_wickra() -> None:
    assert_registered_oracle_match("TripleTopBottom")
