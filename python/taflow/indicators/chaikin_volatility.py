"""Native-backed Chaikin Volatility adapter."""

from typing import Any

import numpy as np

from .._native import ChaikinVolatilityOperator as _Native
from .._series import as_float64_series


class ChaikinVolatility:
    """Compute the rate of change of a smoothed high-low range.

    ``high`` and ``low`` are required equal-length chronological histories and
    may both be empty for a fresh stream. ``timeperiod`` defaults to 10 and
    ``roc_period`` to 10. Rust owns EMA range smoothing, ROC warm-up, and
    aligned output; ``compute`` returns one float array and ``value`` is the
    latest scalar or ``None`` during warm-up. Lifecycle mutators return
    ``self``. The oracle is pandas-ta-classic ``cvol``.
    """

    def __init__(
        self,
        timeperiod: int = 10,
        roc_period: int = 10,
    ) -> None:
        self._state = _Native(int(timeperiod), int(roc_period))

    def append(self, high: float, low: float) -> "ChaikinVolatility":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "ChaikinVolatility":
        """Append equal-length high and low histories."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Chaikin Volatility history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest volatility, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "ChaikinVolatility":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["ChaikinVolatility"]
