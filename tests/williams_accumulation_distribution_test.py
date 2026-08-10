"""External correctness for WilliamsAccumulationDistribution."""

from oracle_assertions import assert_registered_oracle_match


def test_williams_accumulation_distribution_matches_wickra() -> None:
    """Match Wickra WAD across batch and streaming paths."""
    assert_registered_oracle_match("WilliamsAccumulationDistribution")
