"""Descriptive stateful interface for Stochastic RSI."""

from taflow._native import StatefulStochrsi
from typing import Any

import numpy as np


class StochasticRelativeStrengthIndex:
    """Incrementally compute aligned stochastic-RSI fast %K and fast %D

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    StochasticRelativeStrengthIndex
        A persistent native-backed indicator adapter.
    """

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
            Input parameter or configuration value for this operation.
        fast_k_period : object
            Input parameter or configuration value for this operation.
        fast_d_period : object
            Input parameter or configuration value for this operation.
        fast_d_average_type : object
            Input parameter or configuration value for this operation.
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
        self._state.append(_input)
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
        self._state.extend(_input)
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the aligned native output histories

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
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

    def reset(self) -> object:
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
