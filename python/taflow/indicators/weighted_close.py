"""Persistent weighted-close transform."""

from typing import Any

from .._native import WeightedClose as _NativeWeightedClose
from .._price_state import HlcPriceState


class WeightedClose(HlcPriceState):
    """Compute ``(high + low + 2 * close) / 4`` in persistent Rust state.

    Construction creates a fresh empty state. Supply aligned chronological high,
    low, and close series through ``extend`` or ``append``. Output has no
    rolling warm-up and maps to TA-Lib ``WCLPRICE``.
    """

    _native_cls = _NativeWeightedClose

    def append(self, high: float, low: float, close: float) -> "WeightedClose":
        """Append one aligned price tuple and return this indicator."""
        super().append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "WeightedClose":
        """Append aligned price histories and return this indicator."""
        super().extend(high, low, close)
        return self

    def reset(self) -> "WeightedClose":
        """Restore fresh native state and return this indicator."""
        super().reset()
        return self
