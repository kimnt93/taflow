"""Persistent Balance of Power adapter."""

from typing import Any

import numpy as np

from ._native import BalanceOfPower as _NativeBalanceOfPower
from ._series import as_float64_series


class BalanceOfPower:
    """Compute ``(close - open) / (high - low)`` in persistent Rust state.

    The constructor requires aligned chronological open, high, low, and close
    series. Pass four empty arrays for a fresh streaming state. Zero or negative
    high-low ranges produce ``0.0``, matching TA-Lib ``BOP``. There is no
    warm-up.

    Parameters
    ----------
    open : Any
        Chronological opening-price series.
    high : Any
        Chronological high-price series.
    low : Any
        Chronological low-price series.
    close : Any
        Chronological closing-price series.
    """

    def __init__(self, open: Any, high: Any, low: Any, close: Any) -> None:
        self._state = _NativeBalanceOfPower()
        self.extend(open, high, low, close)

    def append(self, open: float, high: float, low: float, close: float) -> "BalanceOfPower":
        """Append one open/high/low/close tuple and return this indicator."""
        self._state.append(float(open), float(high), float(low), float(close))
        return self

    def extend(self, open: Any, high: Any, low: Any, close: Any) -> "BalanceOfPower":
        """Append aligned price histories and return this indicator."""
        self._state.extend(
            as_float64_series(open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned ``float64`` Balance of Power history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest value, or ``None`` before the first tuple."""
        return self._state.value

    def reset(self) -> "BalanceOfPower":
        """Restore fresh native state, clear history, and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed tuples."""
        return len(self._state)
