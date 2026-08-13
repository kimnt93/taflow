"""Canonical native-backed Even Better Sinewave adapter."""

from typing import Any

import numpy as np

from .._native import EvenBetterSinewave as _NativeEvenBetterSinewave
from .._series import as_float64_series


class EvenBetterSinewave:
    """Compute the causal detrended cycle oscillator.

    ``close`` is required and may be empty for a fresh stream. ``length``
    defaults to 40. Rust owns the recurrence and warm-up; ``compute`` returns
    an aligned float array with NaNs before the first known value. Lifecycle
    mutators return ``self`` and ``value`` is the latest scalar or ``None``.
    The correctness oracle is ``pandas_ta_classic.ebsw``.
    """

    def __init__(self, length: int = 40) -> None:
        self._state = _NativeEvenBetterSinewave(int(length))

    def append(self, close: float) -> "EvenBetterSinewave":
        """Append one chronological close and return this adapter."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "EvenBetterSinewave":
        """Append a converted chronological close history."""
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned oscillator history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "EvenBetterSinewave":
        """Reset the state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close values."""
        return len(self._state)


__all__ = ["EvenBetterSinewave"]
