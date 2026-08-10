"""External correctness for RollingPairwiseBeta."""

from oracle_assertions import assert_registered_oracle_match


def test_rolling_pairwise_beta_matches_wickra() -> None:
    """Match Wickra PairwiseBeta across batch and streaming paths."""
    assert_registered_oracle_match("RollingPairwiseBeta")
