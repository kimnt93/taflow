"""Canonical native-backed extended MACD adapter."""

from typing import Any

import numpy as np

from ._native import MovingAverageConvergenceDivergenceExtended as _NativeMovingAverageConvergenceDivergenceExtended
from ._series import as_float64_series


class MovingAverageConvergenceDivergenceExtended:
    """Compute TA-Lib ``MACDEXT`` with independently selected MA types.

    ``values`` is the chronological close series; pass an empty array for a
    fresh stream. Periods default to 12/26/9 and ``fast_matype``,
    ``slow_matype``, and ``signal_matype`` default to TA-Lib code 1 (EMA).
    ``compute`` returns MACD/signal/histogram arrays with NaN warm-up values;
    ``value`` is the latest tuple or ``None``. Lifecycle mutators are fluent.
    Oracle mapping: ``MovingAverageConvergenceDivergenceExtended`` ⇔
    ``MACDEXT``.
    """

    def __init__(
        self,
        values: Any,
        fast_period: int = 12,
        fast_average_type: int = 1,
        slow_period: int = 26,
        slow_average_type: int = 1,
        signal_period: int = 9,
        signal_average_type: int = 1,
    ) -> None:
        self._state = _NativeMovingAverageConvergenceDivergenceExtended(
            fast_period,
            fast_average_type,
            slow_period,
            slow_average_type,
            signal_period,
            signal_average_type,
        )
        self.extend(values)

    def append(self, value: float) -> "MovingAverageConvergenceDivergenceExtended":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MovingAverageConvergenceDivergenceExtended":
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergenceExtended":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["MovingAverageConvergenceDivergenceExtended"]
