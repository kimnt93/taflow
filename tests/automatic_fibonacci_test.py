from tests.oracle_assertions import assert_registered_oracle_match


def test_automatic_fibonacci_matches_wickra() -> None:
    assert_registered_oracle_match("AutomaticFibonacci")
