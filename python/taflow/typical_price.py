"""Canonical typical-price adapter."""

from ._native import StatefulTypprice
from ._price_state import HlcPriceState


class TypicalPrice(HlcPriceState):
    """Compute typical price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TypicalPrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTypprice

    def append(self, high: object, low: object, close: object) -> "TypicalPrice":
        """Append one observation and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: object, low: object, close: object) -> "TypicalPrice":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "TypicalPrice":
        """Reset native state and return this indicator."""
        super().reset()
        return self
