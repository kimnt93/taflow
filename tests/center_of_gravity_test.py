from oracle_assertions import assert_registered_oracle_match


def test_center_of_gravity_matches_wickra() -> None:
    assert_registered_oracle_match("CenterOfGravity")
