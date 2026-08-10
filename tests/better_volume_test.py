"""External correctness for BetterVolume."""

from oracle_assertions import assert_registered_oracle_match


def test_better_volume_matches_wickra() -> None:
    """Match Wickra BetterVolume through the canonical Python class."""
    assert_registered_oracle_match("BetterVolume")
