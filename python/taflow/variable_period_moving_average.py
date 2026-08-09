"""Native adapter for a variable-period moving average."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import VariablePeriodMovingAverage as _NativeVariablePeriodMovingAverage
from ._series import as_float64_series


class VariablePeriodMovingAverage:
    """Compute a moving average whose trailing period changes on each bar.

    Parameters
    ----------
    values : array-like
        Initial chronological input values. Pass an empty aligned series to
        create a fresh streaming state.
    periods : array-like
        Per-bar periods aligned with ``values``. Values are truncated to an
        integer and clamped to ``[min_period, max_period]`` like TA-Lib MAVP.
    min_period : int, default 2
        Smallest permitted moving-average period.
    max_period : int, default 30
        Largest permitted period and the basis of global warm-up.
    average_type : int, default 0
        TA-Lib moving-average type code from 0 through 8.

    Notes
    -----
    The independent oracle is TA-Lib ``MAVP``. Rust owns period coercion,
    warm-up, rolling state, and aligned NaN output. ``append``, ``extend``, and
    ``reset`` mutate this adapter and return it for fluent use.
    """

    def __init__(
        self,
        values: Any,
        periods: Any,
        min_period: int = 2,
        max_period: int = 30,
        average_type: int = 0,
    ) -> None:
        """Create the native state and process the required aligned inputs."""
        self._state = _NativeVariablePeriodMovingAverage(
            min_period, max_period, average_type
        )
        self.extend(values, periods)

    def append(self, value: float, period: float) -> "VariablePeriodMovingAverage":
        """Append one value and its period to the native Rust state.

        Parameters
        ----------
        value : float
            Next chronological input value.
        period : float
            Period for this bar; Rust truncates and clamps it to the configured
            range.

        Returns
        -------
        VariablePeriodMovingAverage
            This updated adapter.
        """
        self._state.append(float(value), float(period))
        return self

    def extend(self, values: Any, periods: Any) -> "VariablePeriodMovingAverage":
        """Append aligned chronological value and period series.

        Parameters
        ----------
        values : array-like
            Values to process.
        periods : array-like
            One requested period per value. Misaligned inputs are rejected by
            Rust before state mutation.

        Returns
        -------
        VariablePeriodMovingAverage
            This updated adapter.
        """
        self._state.extend(
            as_float64_series(values),
            as_float64_series(periods),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned native history.

        Returns
        -------
        numpy.ndarray
            One value per processed bar, with NaN in warm-up positions.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest warmed native result.

        Returns
        -------
        float or None
            Latest moving average, or ``None`` during global warm-up.
        """
        return self._state.value

    def reset(self) -> "VariablePeriodMovingAverage":
        """Reset native state and aligned history without changing configuration.

        Returns
        -------
        VariablePeriodMovingAverage
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["VariablePeriodMovingAverage"]
