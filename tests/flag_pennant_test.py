from tests.oracle_assertions import assert_registered_oracle_match


def test_flag_pennant_matches_wickra() -> None:
    assert_registered_oracle_match("FlagPennant")
