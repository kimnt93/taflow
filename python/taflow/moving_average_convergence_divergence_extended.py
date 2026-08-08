"""Descriptive stateful interface for extended MACD."""

from taflow._native import StatefulMacdExt
from typing import Any

import numpy as np


class MovingAverageConvergenceDivergenceExtended:
    """Incrementally compute MACDEXT with independently selected MA types

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MovingAverageConvergenceDivergenceExtended
        A persistent native-backed indicator adapter.
    """

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
