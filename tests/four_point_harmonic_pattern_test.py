from tests.oracle_assertions import assert_registered_oracle_match


def test_four_point_harmonic_pattern_matches_wickra() -> None:
    assert_registered_oracle_match("FourPointHarmonicPattern")
