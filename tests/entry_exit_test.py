import numpy as np

from taflow import EntryExit


def test_tracks_entry_exit_position() -> None:
    indicator = EntryExit(
        np.array([False, True, False, True]),
        np.array([False, False, True, True]),
    )
    np.testing.assert_array_equal(indicator.compute(), [0.0, 1.0, -1.0, -1.0])
    assert indicator.reset().value is None
