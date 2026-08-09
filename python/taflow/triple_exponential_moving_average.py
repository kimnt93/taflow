"""Canonical native-backed Triple Exponential Moving Average adapter."""

from typing import Any

import numpy as np

from ._native import TripleExponentialMovingAverage as _NativeTripleExponentialMovingAverage
from ._series import as_float64_series


class TripleExponentialMovingAverage:
    """Compute TEMA from a close series.

    Parameters
    ----------
    values : array-like
        Chronological input values; an empty array creates a fresh stream.
    timeperiod : int, default 30
        EMA period. Values below 2 are rejected by the native state.

    Returns
    -------
    TripleExponentialMovingAverage
        Native-backed state. ``compute`` returns one aligned NumPy array,
        with NaN during the three-EMA warm-up; ``value`` is the latest scalar
        or ``None``. ``append``, ``extend``, and ``reset`` return ``self``.

    Notes
    -----
    The canonical class maps to TA-Lib ``TEMA``. Rust owns all recurrence,
    warm-up, and bulk processing; this adapter only converts input series.
    """

    def __init__(self, values: Any, timeperiod: int = 30) -> None:
        self._state = _NativeTripleExponentialMovingAverage(timeperiod)
        self.extend(values)

    def append(self, value: float) -> "TripleExponentialMovingAverage":
        """Append one value and return this indicator for fluent chaining."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TripleExponentialMovingAverage":
        """Append a chronological values series and return this indicator."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned TEMA history, including NaN warm-up values."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest TEMA value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "TripleExponentialMovingAverage":
        """Reset to a fresh state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["TripleExponentialMovingAverage"]
