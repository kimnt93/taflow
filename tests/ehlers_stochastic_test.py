from oracle_assertions import assert_registered_oracle_match


def test_ehlers_stochastic_matches_wickra() -> None:
    assert_registered_oracle_match("EhlersStochastic")
