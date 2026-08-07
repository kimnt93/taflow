"""Descriptive stateful interface for extended MACD."""

from taflow._native import StatefulMacdExt
from typing import Any

import numpy as np


class MovingAverageConvergenceDivergenceExtended:
    """Incrementally compute MACDEXT with independently selected MA types."""

    def __init__(
        self,
        fast_period=12,
        fast_average_type=1,
        slow_period=26,
        slow_average_type=1,
        signal_period=9,
        signal_average_type=1,
        _input: Any | None = None,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        fast_period : object
            Input series, scalar parameter, or configuration value for this operation.
        fast_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_period : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        signal_period : object
            Input series, scalar parameter, or configuration value for this operation.
        signal_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulMacdExt(
            fast_period,
            fast_average_type,
            slow_period,
            slow_average_type,
            signal_period,
            signal_average_type,
        )
        self._values: list[tuple[float, ...]] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(_input)
        self._values.append(
            (np.nan, np.nan, np.nan) if result is None else tuple(result)
        )
        return self

    def extend(self, _input):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(_input)
        arrays = [np.asarray(item, dtype=np.float64) for item in result]
        self._values.extend(zip(*arrays))
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the aligned native output histories."""
        if not self._values:
            empty = np.empty(0, dtype=np.float64)
            return tuple(empty.copy() for _ in range(3))
        return tuple(
            np.asarray(values, dtype=np.float64) for values in zip(*self._values)
        )

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
