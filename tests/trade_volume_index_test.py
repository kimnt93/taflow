"""External correctness for TradeVolumeIndex."""

from oracle_assertions import assert_registered_oracle_match


def test_trade_volume_index_matches_wickra() -> None:
    """Match Wickra TradeVolumeIndex, including minimum-tick direction."""
    assert_registered_oracle_match("TradeVolumeIndex")
