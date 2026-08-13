"""Canonical native-backed fixed-parameter MACD adapter."""

from typing import Any

import numpy as np

from .._native import MovingAverageConvergenceDivergenceFixed as _NativeMovingAverageConvergenceDivergenceFixed
from .._series import as_float64_series


class MovingAverageConvergenceDivergenceFixed:
    """Compute TA-Lib ``MACDFIX`` with fixed 12/26 fast and slow constants.

    ``values`` is the chronological close series; empty input creates a fresh
    state. ``signal_period`` defaults to 9. ``compute`` returns aligned
    MACD/signal/histogram arrays, with NaN warm-up positions, and ``value`` is
    the latest tuple or ``None`` during warm-up. ``append``, ``extend``, and
    ``reset`` mutate and return this adapter. Oracle: ``MACDFIX``.
    """

    def __init__(self, signal_period: int = 9) -> None:
        self._state = _NativeMovingAverageConvergenceDivergenceFixed(signal_period)

    def append(self, value: float) -> "MovingAverageConvergenceDivergenceFixed":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MovingAverageConvergenceDivergenceFixed":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergenceFixed":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["MovingAverageConvergenceDivergenceFixed"]
