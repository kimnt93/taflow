"""Native-backed Hull moving-average adapter."""

from typing import Any

import numpy as np

from ._native import HmaOperator as _Native
from ._series import as_float64_series


class HullMovingAverage:
    """Compute the causal Hull moving average.

    ``_input`` is the required chronological price series and may be empty for
    a fresh stream. ``timeperiod`` defaults to 10. Rust owns the weighted
    moving-average cascade, warm-up, and aligned history. ``compute`` returns
    one float array, ``value`` is the latest scalar or ``None`` while warming
    up, and lifecycle mutators return ``self``. The oracle is pandas-ta-classic
    ``hma``.
    """

    def __init__(self, _input: Any, timeperiod: int = 10) -> None:
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "HullMovingAverage":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "HullMovingAverage":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Hull moving-average history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest average, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "HullMovingAverage":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return self._length


__all__ = ["HullMovingAverage"]
