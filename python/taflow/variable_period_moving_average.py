"""Descriptive stateful interface for a variable-period moving average."""

from taflow._native import StatefulMavp
from typing import Any

import numpy as np


class VariablePeriodMovingAverage:
    """Incrementally compute MAVP from values and per-bar periods

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    VariablePeriodMovingAverage
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        min_period: int = 2,
        max_period: int = 30,
        average_type: int = 0,
        _input: Any | None = None,
        periods: Any | None = None,
    ) -> None:
        """Create MAVP with optional values and per-bar periods."""
        self._state = StatefulMavp(min_period, max_period, average_type)
        self._values: list[float] = []
        if _input is not None or periods is not None:
            self.extend(_input, periods)

    def append(self, _input: float, period: int) -> "VariablePeriodMovingAverage":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        period : object
            Trailing window length in bars.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(float(_input), int(period))
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, _input: Any, periods: Any) -> "VariablePeriodMovingAverage":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        periods : object
            Lookback periods used by the estimator.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(
            np.asarray(_input, dtype=np.float64),
            np.asarray(periods, dtype=np.float64),
        )
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned variable-period moving-average values

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "VariablePeriodMovingAverage":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
