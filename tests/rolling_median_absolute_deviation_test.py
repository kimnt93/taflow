"""External correctness for RollingMedianAbsoluteDeviation."""

from oracle_assertions import assert_registered_oracle_match


def test_rolling_median_absolute_deviation_matches_wickra() -> None:
    """Match Wickra MedianAbsoluteDeviation through the public class API."""
    assert_registered_oracle_match("RollingMedianAbsoluteDeviation")
