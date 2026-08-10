"""External correctness for RollingAverageDrawdown."""

from oracle_assertions import assert_registered_oracle_match


def test_rolling_average_drawdown_matches_wickra() -> None:
    """Match Wickra AverageDrawdown across batch and streaming paths."""
    assert_registered_oracle_match("RollingAverageDrawdown")
