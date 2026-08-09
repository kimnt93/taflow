"""Canonical average-price adapter."""

from ._native import StatefulAvgprice
from ._price_state import OhlcPriceState


class AveragePrice(OhlcPriceState):
    """Compute average price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AveragePrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAvgprice

    def append(self, _open: object, high: object, low: object, close: object) -> "AveragePrice":
        """Append one observation and return this indicator."""
        super().append(_open, high, low, close)
        return self

    def extend(self, _open: object, high: object, low: object, close: object) -> "AveragePrice":
        """Append aligned histories and return this indicator."""
        super().extend(_open, high, low, close)
        return self

    def reset(self) -> "AveragePrice":
        """Reset native state and return this indicator."""
        super().reset()
        return self
