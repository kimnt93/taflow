from oracle_assertions import assert_registered_oracle_match


def test_overnight_intraday_return_matches_wickra() -> None:
    assert_registered_oracle_match("OvernightIntradayReturn")
