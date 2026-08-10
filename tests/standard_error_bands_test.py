from tests.oracle_assertions import assert_registered_oracle_match


def test_standard_error_bands_matches_wickra() -> None:
    assert_registered_oracle_match("StandardErrorBands")
