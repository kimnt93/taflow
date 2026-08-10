from tests.oracle_assertions import assert_registered_oracle_match


def test_mc_clellan_oscillator_matches_wickra() -> None:
    assert_registered_oracle_match("McClellanOscillator")
