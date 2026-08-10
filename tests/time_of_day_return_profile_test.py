from tests.oracle_assertions import assert_registered_oracle_match


def test_time_of_day_return_profile_matches_wickra() -> None:
    assert_registered_oracle_match("TimeOfDayReturnProfile")
