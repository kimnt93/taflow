"""Descriptive stateful interface for Bollinger Bands."""

from taflow._native import StatefulBbands
from typing import Any

import numpy as np


class BollingerBands:
    """Incrementally compute upper, middle, and lower Bollinger Bands

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    BollingerBands
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        values: Any,
        period: object = 5,
        deviations_up: object = 2.0,
        deviations_down: object = 2.0,
        moving_average_type: object = 0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        period : object
            Trailing window length in bars.
        deviations_up : object
            Input parameter or configuration value for this operation.
        deviations_down : object
            Input parameter or configuration value for this operation.
        moving_average_type : object
            Input parameter or configuration value for this operation.
        values : object
            Input values processed in chronological order.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulBbands(
            period, deviations_up, deviations_down, moving_average_type
        )
        if values is not None:
            self.extend(values)

    def append(self, value: float) -> "BollingerBands":
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

    def extend(self, values: Any) -> "BollingerBands":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(values)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower band histories.

        Returns
        -------
        tuple of numpy.ndarray
            Three same-length arrays in upper, middle, and lower order.
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

    def reset(self) -> "BollingerBands":
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
