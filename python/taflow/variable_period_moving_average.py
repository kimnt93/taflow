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
        _input: Any,
        periods: Any,
        min_period: int = 2,
        max_period: int = 30,
        average_type: int = 0,
    ) -> None:
        """Create MAVP with optional values and per-bar periods."""
        self._state = StatefulMavp(min_period, max_period, average_type)
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
        self._state.append(float(_input), int(period))
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
        self._state.extend(
            np.asarray(_input, dtype=np.float64),
            np.asarray(periods, dtype=np.float64),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

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
        return self

    def __len__(self) -> int:
        return len(self._state)
