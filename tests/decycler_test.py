from oracle_assertions import assert_registered_oracle_match


def test_decycler_matches_wickra() -> None:
    assert_registered_oracle_match("Decycler")
