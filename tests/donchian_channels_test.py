import numpy as np

from taflow.donchian_channels import DonchianChannels


def test_donchian_channels_lifecycle_and_reset():
    state = DonchianChannels(np.array([], dtype=float), np.array([], dtype=float), timeperiod=2)
    state.extend([10.0, 12.0], [8.0, 7.0])
    np.testing.assert_allclose(state.compute()[0], [np.nan, 12.0], equal_nan=True)
    state.reset()
    assert state.value is None
