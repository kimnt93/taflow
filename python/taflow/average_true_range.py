"""Canonical Average True Range adapter."""
from typing import Any

from ._native import StatefulAtr
from ._ohlc_state import OhlcStateAdapter


class AverageTrueRange(OhlcStateAdapter):
    """Compute Wilder's average true range through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AverageTrueRange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAtr

    def append(self, high: float, low: float, close: float) -> "AverageTrueRange":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "AverageTrueRange":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "AverageTrueRange":
        """Reset native state and return this indicator."""
        super().reset()
        return self
