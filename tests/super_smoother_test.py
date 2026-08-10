from oracle_assertions import assert_registered_oracle_match


def test_super_smoother_matches_wickra() -> None:
    assert_registered_oracle_match("SuperSmoother")
