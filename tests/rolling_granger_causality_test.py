"""External correctness for RollingGrangerCausality."""

from oracle_assertions import assert_registered_oracle_match


def test_rolling_granger_causality_matches_wickra() -> None:
    """Match Wickra GrangerCausality across batch and streaming paths."""
    assert_registered_oracle_match("RollingGrangerCausality")
