"""Canonical median-price adapter."""

from ._native import StatefulMedprice
from ._price_state import HlPriceState


class MedianPrice(HlPriceState):
    """Compute median price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MedianPrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMedprice

    def append(self, high: object, low: object) -> "MedianPrice":
        """Append one observation and return this indicator."""
        super().append(high, low)
        return self

    def extend(self, high: object, low: object) -> "MedianPrice":
        """Append aligned histories and return this indicator."""
        super().extend(high, low)
        return self

    def reset(self) -> "MedianPrice":
        """Reset native state and return this indicator."""
        super().reset()
        return self
