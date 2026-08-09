"""Canonical Williams %R adapter."""
from typing import Any

from ._native import StatefulWillr
from ._ohlc_state import OhlcStateAdapter


class WilliamsPercentR(OhlcStateAdapter):
    """Compute Williams Percent R from aligned high, low, and close prices.

    Parameters
    ----------
    high, low, close : array-like
        Initial aligned price histories. Later bars are supplied through
        ``append`` or ``extend``.
    timeperiod : int, optional
        Trailing lookback used by the native kernel.

    Returns
    -------
    WilliamsPercentR
        A persistent native-backed indicator state.
    """

    _native_cls = StatefulWillr

    def append(self, high: float, low: float, close: float) -> "WilliamsPercentR":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "WilliamsPercentR":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "WilliamsPercentR":
        """Reset native state and return this indicator."""
        super().reset()
        return self
