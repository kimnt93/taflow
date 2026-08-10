"""External correctness for TimeSegmentedVolume."""

from oracle_assertions import assert_registered_oracle_match


def test_time_segmented_volume_matches_wickra() -> None:
    """Match Wickra TSV through the canonical Python class."""
    assert_registered_oracle_match("TimeSegmentedVolume")
