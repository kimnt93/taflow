from tests.oracle_assertions import assert_registered_oracle_match


def test_fibonacci_time_zones_matches_wickra() -> None:
    assert_registered_oracle_match("FibonacciTimeZones")
