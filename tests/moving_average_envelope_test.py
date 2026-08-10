from tests.oracle_assertions import assert_registered_oracle_match


def test_moving_average_envelope_matches_wickra() -> None:
    assert_registered_oracle_match("MovingAverageEnvelope")
