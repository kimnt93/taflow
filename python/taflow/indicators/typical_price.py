"""Persistent typical-price transform."""

from typing import Any

from .._native import TypicalPrice as _NativeTypicalPrice
from .._price_state import HlcPriceState


class TypicalPrice(HlcPriceState):
    """Compute ``(high + low + close) / 3`` in persistent Rust state.

    The constructor requires the aligned chronological high, low, close
    series. Pass empty aligned arrays for a fresh streaming state. Output has no
    rolling warm-up and maps to TA-Lib ``TYPPRICE``.
    """

    _native_cls = _NativeTypicalPrice

    def append(self, high: float, low: float, close: float) -> "TypicalPrice":
        """Append one aligned price tuple and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "TypicalPrice":
        """Append aligned price histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "TypicalPrice":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
