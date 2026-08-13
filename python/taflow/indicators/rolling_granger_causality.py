"""Public adapter for native rolling Granger causality."""

from typing import Any

import numpy as np

from .._native import RollingGrangerCausality as _Native
from .._series import as_float64_series


class RollingGrangerCausality:
    """Test whether past predictor values improve forecasts of a dependent series.

    The output is the rolling F-statistic comparing an autoregression of
    ``dependent`` with one augmented by ``predictor`` lags. Larger non-negative
    values mean stronger predictive information; this is not structural
    causation. Warm-up lasts ``period - 1`` bars. The oracle mapping is Wickra
    ``GrangerCausality``.

    Args:
        dependent: Chronological series being predicted.
        predictor: Chronological candidate explanatory series.
        period: Regression lookback, default 60.
        lag: Autoregressive order, default 1.

    Raises:
        ValueError: If inputs differ in length, lag is zero, or period is less
            than ``3 * lag + 2``.
    """

    def __init__(self, period: int = 60, lag: int = 1) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period, lag)

    def append(self, dependent: float, predictor: float) -> "RollingGrangerCausality":
        """Append one aligned observation pair and return this instance."""
        self._state.append(float(dependent), float(predictor))
        return self

    def extend(self, dependent: Any, predictor: Any) -> "RollingGrangerCausality":
        """Append aligned dependent and predictor histories and return self."""
        dependent_series = as_float64_series(dependent)
        predictor_series = as_float64_series(predictor)
        if len(dependent_series) != len(predictor_series):
            raise ValueError("dependent and predictor must have equal lengths")
        self._state.extend(dependent_series, predictor_series)
        return self

    @property
    def value(self) -> float | None:
        """Return latest F-statistic, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned F-statistics with warm-up represented by ``NaN``."""
        return self._state.compute()

    def reset(self) -> "RollingGrangerCausality":
        """Clear regression history and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of pairs stored by native state."""
        return len(self._state)
