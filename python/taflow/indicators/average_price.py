"""Persistent average-price transform."""

from typing import Any

from .._native import AveragePrice as _NativeAveragePrice
from .._price_state import OhlcPriceState


class AveragePrice(OhlcPriceState):
    """Compute ``(open + high + low + close) / 4`` in persistent Rust state.

    Construction creates a fresh empty state. Supply aligned chronological open,
    high, low, and close series through ``extend`` or ``append``. Output has no
    rolling warm-up and maps to TA-Lib ``AVGPRICE``.
    """

    _native_cls = _NativeAveragePrice

    def append(self, open: float, high: float, low: float, close: float) -> "AveragePrice":
        """Append one aligned price tuple and return this indicator."""
        super().append(open, high, low, close)
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "AveragePrice":
        """Append aligned price histories and return this indicator."""
        super().extend(open, high, low, close)
        return self

    def reset(self) -> "AveragePrice":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
