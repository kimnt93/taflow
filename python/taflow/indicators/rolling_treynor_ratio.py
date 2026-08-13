"""Public adapter for the native rolling Treynor ratio state."""

from typing import Any

import numpy as np

from .._native import RollingTreynorRatio as _Native
from .._series import as_float64_series


class RollingTreynorRatio:
    """Measure rolling mean asset return relative to benchmark beta.

    Rust derives beta from population covariance and benchmark variance, then
    divides mean asset return by that beta. Output remains ``NaN`` until the
    window is full. This maps to Wickra ``TreynorRatio`` 0.9.9 with its default
    zero risk-free rate; TA-Lib has no direct equivalent.

    Args:
        values: Chronological asset-return series.
        benchmark: Initial benchmark-return series aligned with ``values``.
        timeperiod: Rolling window length. Defaults to 14.

    Raises:
        ValueError: If the inputs are misaligned or the period is zero.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(int(timeperiod))

    def append(
        self,
        value: float,
        benchmark: float,
    ) -> "RollingTreynorRatio":
        """Append one asset/benchmark return pair and return this instance."""
        self._state.append(float(value), float(benchmark))
        return self

    def extend(self, values: Any, benchmark: Any) -> "RollingTreynorRatio":
        """Append aligned asset and benchmark return series.

        Raises:
            ValueError: If the two series have different lengths.
        """
        value_series = as_float64_series(values)
        benchmark_series = as_float64_series(benchmark)
        if len(value_series) != len(benchmark_series):
            raise ValueError("values and benchmark must have equal lengths")
        self._state.extend(value_series, benchmark_series)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest ratio, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned ratio values, including warm-up ``NaN``."""
        return self._state.compute()

    def reset(self) -> "RollingTreynorRatio":
        """Restore fresh-state behavior and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of aligned pairs processed by Rust."""
        return len(self._state)


__all__ = ["RollingTreynorRatio"]
