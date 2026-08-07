"""Descriptive stateful interface for extended MACD."""

from taflow._native import StatefulMacdExt
from typing import Any

import numpy as np


class MovingAverageConvergenceDivergenceExtended:
    """Incrementally compute MACDEXT with independently selected MA types."""

    def __init__(
        self,
        fast_period: object = 12,
        fast_average_type: object = 1,
        slow_period: object = 26,
        slow_average_type: object = 1,
        signal_period: object = 9,
        signal_average_type: object = 1,
        _input: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        fast_period : object
            Fast smoothing length in bars.
        fast_average_type : object
            Input parameter or configuration value for this operation.
        slow_period : object
            Slow smoothing length in bars.
        slow_average_type : object
            Input parameter or configuration value for this operation.
        signal_period : object
            Signal smoothing length in bars.
        signal_average_type : object
            Input parameter or configuration value for this operation.
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
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

    def append(self, _input: object) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(_input)
        self._values.append(
            (np.nan, np.nan, np.nan) if result is None else tuple(result)
        )
        return self

    def extend(self, _input: object) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
