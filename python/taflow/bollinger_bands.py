"""Native-backed Bollinger Bands adapter."""

from typing import Any

import numpy as np

from ._native import BollingerBands as _NativeBollingerBands
from ._series import as_float64_series


class BollingerBands:
    """Compute Bollinger bands from one causal price history.

    ``values`` is required and may be empty for a fresh stream. ``period``
    defaults to 5, ``deviations_up`` and ``deviations_down`` default to 2.0,
    and ``moving_average_type`` defaults to 0 (SMA, matching TA-Lib
    ``BBANDS``). Rust owns the rolling average, deviation, and warm-up; the
    output order is ``(upper, middle, lower)`` with NaN until the period is
    available. Lifecycle mutators return ``self``.
    """

    def __init__(
        self,
        values: Any,
        period: int = 5,
        deviations_up: float = 2.0,
        deviations_down: float = 2.0,
        moving_average_type: int = 0,
    ) -> None:
        self._state = _NativeBollingerBands(
            int(period),
            float(deviations_up),
            float(deviations_down),
            int(moving_average_type),
        )
        self.extend(values)

    def append(self, value: float) -> "BollingerBands":
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "BollingerBands":
        converted = as_float64_series(values)
        self._state.extend(converted)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned ``(upper, middle, lower)`` histories."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest band tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "BollingerBands":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


__all__ = ["BollingerBands"]
