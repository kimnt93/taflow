"""External correctness for RollingStandardError."""

from oracle_assertions import assert_registered_oracle_match


def test_rolling_standard_error_matches_wickra() -> None:
    """Match Wickra StandardError across all lifecycle paths."""
    assert_registered_oracle_match("RollingStandardError")
