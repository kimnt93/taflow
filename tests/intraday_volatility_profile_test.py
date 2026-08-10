from tests.oracle_assertions import assert_registered_oracle_match


def test_intraday_volatility_profile_matches_wickra() -> None:
    assert_registered_oracle_match("IntradayVolatilityProfile")
