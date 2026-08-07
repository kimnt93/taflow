"""Descriptive stateful interface for Bollinger Bands."""

from taflow._native import StatefulBbands
from typing import Any


class BollingerBands:
    """Incrementally compute upper, middle, and lower Bollinger Bands."""

    def __init__(
        self,
        period=5,
        deviations_up=2.0,
        deviations_down=2.0,
        moving_average_type=0,
        values: Any | None = None,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        period : object
            Input series, scalar parameter, or configuration value for this operation.
        deviations_up : object
            Input series, scalar parameter, or configuration value for this operation.
        deviations_down : object
            Input series, scalar parameter, or configuration value for this operation.
        moving_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        values : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulBbands(
            period, deviations_up, deviations_down, moving_average_type
        )
        if values is not None:
            self.extend(values)

    def append(self, value):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(value)

    def extend(self, values):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(values)

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
