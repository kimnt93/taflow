import numpy as np
import talib

from taflow import CandleAdvanceBlock


def test_advance_block_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleAdvanceBlock().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    np.testing.assert_array_equal(
        indicator.compute(),
        talib.CDLADVANCEBLOCK(values, values + 2.0, values - 2.0, values + 0.5),
    )
    indicator.reset()
    assert indicator.value is None
