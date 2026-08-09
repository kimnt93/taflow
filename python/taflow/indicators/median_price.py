"""Persistent median-price transform."""

from typing import Any

from .._native import MedianPrice as _NativeMedianPrice
from .._price_state import HlPriceState


class MedianPrice(HlPriceState):
    """Compute ``(high + low) / 2`` in persistent Rust state.

    The constructor requires the aligned chronological high, low
    series. Pass empty aligned arrays for a fresh streaming state. Output has no
    rolling warm-up and maps to TA-Lib ``MEDPRICE``.
    """

    _native_cls = _NativeMedianPrice

    def append(self, high: float, low: float) -> "MedianPrice":
        """Append one aligned price tuple and return this indicator."""
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "MedianPrice":
        """Append aligned price histories and return this indicator."""
        super().extend(high, low)
        return self

    def reset(self) -> "MedianPrice":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
