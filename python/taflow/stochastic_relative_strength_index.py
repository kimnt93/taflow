"""Descriptive stateful interface for Stochastic RSI."""

from taflow._native import StatefulStochrsi
from typing import Any

import numpy as np


class StochasticRelativeStrengthIndex:
    """Incrementally compute aligned stochastic-RSI fast %K and fast %D."""

    def __init__(
        self,
        time_period: object = 14,
        fast_k_period: object = 5,
        fast_d_period: object = 3,
        fast_d_average_type: object = 0,
        _input: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        time_period : object
            Values or parameters consumed by this operation.
        fast_k_period : object
            Values or parameters consumed by this operation.
        fast_d_period : object
            Values or parameters consumed by this operation.
        fast_d_average_type : object
            Values or parameters consumed by this operation.
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulStochrsi(
            time_period,
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(_input)
        self._values.append((np.nan, np.nan) if result is None else tuple(result))
        return self

    def extend(self, _input: object) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

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
            return tuple(empty.copy() for _ in range(2))
        return tuple(
            np.asarray(values, dtype=np.float64) for values in zip(*self._values)
        )

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
