"""Canonical native-backed Moving Average Convergence/Divergence adapter."""

from typing import Any

import numpy as np

from .._native import MovingAverageConvergenceDivergence as _NativeMovingAverageConvergenceDivergence
from .._series import as_float64_series


class MovingAverageConvergenceDivergence:
    """Compute TA-Lib ``MACD`` as aligned MACD, signal, and histogram arrays.

    Supply the chronological ``values`` close series through ``extend``. ``fast_period``, ``slow_period``, and ``signal_period``
    default to 12, 26, and 9 and must be valid positive periods. Scalar
    ``value`` is ``None`` until the signal warm-up completes. ``compute``
    returns a tuple in MACD/signal/histogram order with NaN warm-up entries.
    All lifecycle methods mutate and return this adapter. The independent
    oracle mapping is ``MovingAverageConvergenceDivergence`` ⇔ TA-Lib
    ``MACD``.
    """

    def __init__(
        self,
        fast_period: int = 12,
        slow_period: int = 26,
        signal_period: int = 9,
    ) -> None:
        self._state = _NativeMovingAverageConvergenceDivergence(
            fast_period, slow_period, signal_period
        )

    def append(self, value: float) -> "MovingAverageConvergenceDivergence":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MovingAverageConvergenceDivergence":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergence":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["MovingAverageConvergenceDivergence"]
