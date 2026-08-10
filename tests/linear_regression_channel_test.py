from tests.oracle_assertions import assert_registered_oracle_match


def test_linear_regression_channel_matches_wickra() -> None:
    assert_registered_oracle_match("LinearRegressionChannel")
