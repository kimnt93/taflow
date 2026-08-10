from tests.oracle_assertions import assert_registered_oracle_match


def test_double_bollinger_bands_matches_wickra() -> None:
    assert_registered_oracle_match("DoubleBollingerBands")
