from tests.oracle_assertions import assert_registered_oracle_match


def test_day_of_week_return_profile_matches_wickra() -> None:
    assert_registered_oracle_match("DayOfWeekReturnProfile")
