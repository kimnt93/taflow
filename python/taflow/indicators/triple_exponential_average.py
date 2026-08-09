"""Canonical native-backed Tillson T3 adapter."""

from typing import Any

import numpy as np

from .._native import TripleExponentialAverage as _NativeTripleExponentialAverage
from .._series import as_float64_series


class TripleExponentialAverage:
    """Compute Tillson T3 from a chronological input series.

    Parameters
    ----------
    values : array-like
        Input values; an empty array creates a fresh stream.
    timeperiod : int, default 5
        Period for each of the six seeded EMAs; values below 2 are rejected.
    volume_factor : float, default 0.7
        T3 volume factor used for the four cascade coefficients.

    Returns
    -------
    TripleExponentialAverage
        Native-backed state. ``compute`` returns aligned output with NaN
        warm-up values, ``value`` is the latest scalar or ``None``, and all
        lifecycle mutators return ``self``.

    Notes
    -----
    The canonical class maps to TA-Lib ``T3``. Rust owns the six-EMA
    recurrence, warm-up, and bulk processing.
    """

    def __init__(
        self,
        values: Any,
        timeperiod: int = 5,
        volume_factor: float = 0.7,
    ) -> None:
        self._state = _NativeTripleExponentialAverage(timeperiod, volume_factor)
        self.extend(values)

    def append(self, value: float) -> "TripleExponentialAverage":
        """Append one value and return this indicator for fluent chaining."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TripleExponentialAverage":
        """Append a chronological values series and return this indicator."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned T3 history, including NaN warm-up values."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest T3 value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "TripleExponentialAverage":
        """Reset to a fresh state and return this indicator."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["TripleExponentialAverage"]
