from tests.oracle_assertions import assert_registered_oracle_match


def test_percent_above_moving_average_matches_wickra() -> None:
    assert_registered_oracle_match("PercentAboveMovingAverage")
