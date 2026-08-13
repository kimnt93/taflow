import numpy as np

from taflow import CandleUpDownSideGapThreeMethods


def test_candle_up_down_side_gap_three_methods_lifecycle() -> None:
    open_ = np.arange(12.0)
    high = open_ + 2.0
    low = open_ - 1.0
    close = open_ + 1.0
    indicator = CandleUpDownSideGapThreeMethods().extend(open_, high, low, close)
    expected = indicator.compute()
    indicator.reset().extend(open_, high, low, close)
    np.testing.assert_array_equal(indicator.compute(), expected)
    assert len(indicator) == len(open_)
