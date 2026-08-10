from tests.oracle_assertions import assert_registered_oracle_match


def test_fibonacci_projection_matches_wickra() -> None:
    assert_registered_oracle_match("FibonacciProjection")
