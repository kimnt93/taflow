"""Persistent Exponential Moving Average indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import ExponentialMovingAverage as _NativeExponentialMovingAverage
from ._series import as_float64_series


class ExponentialMovingAverage:
    """Compute EMA history once, then continue it with new observations."""

    def __init__(
        self,
        _input: Any | None = None,
        timeperiod: int = 30,
        *,
        column: str | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.
        timeperiod : object
            Trailing window length in bars.
        column : object
            Input parameter or configuration value for this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _NativeExponentialMovingAverage(timeperiod)
        if _input is not None:
            self.extend(_input, column=column)

    def append(self, value: float) -> "ExponentialMovingAverage":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input value processed at each bar.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ExponentialMovingAverage":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.
        column : object
            Input parameter or configuration value for this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(values, column=column))
        return self

    def compute(self) -> np.ndarray:
        """Return every aligned result accumulated by this object."""

        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest warm value without materializing history."""

        return self._state.value

    @property
    def timeperiod(self) -> int:
        """Execute the timeperiod operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.timeperiod

    def reset(self) -> "ExponentialMovingAverage":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Execute the __len__ operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return len(self._state)


__all__ = ["ExponentialMovingAverage"]
