"""External correctness for TwiggsMoneyFlow."""

from oracle_assertions import assert_registered_oracle_match


def test_twiggs_money_flow_matches_wickra() -> None:
    """Match Wickra TwiggsMoneyFlow across all lifecycle paths."""
    assert_registered_oracle_match("TwiggsMoneyFlow")
