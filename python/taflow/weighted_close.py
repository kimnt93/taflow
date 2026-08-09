"""Canonical weighted-close adapter."""

from ._native import StatefulWclprice
from ._price_state import HlcPriceState


class WeightedClose(HlcPriceState):
    """Compute weighted close through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    WeightedClose
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulWclprice

    def append(self, high: object, low: object, close: object) -> "WeightedClose":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: object, low: object, close: object) -> "WeightedClose":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "WeightedClose":
        """Reset native state and return this indicator."""
        super().reset()
        return self
