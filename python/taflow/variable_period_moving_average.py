"""Descriptive stateful interface for a variable-period moving average."""

from taflow._native import StatefulMavp
from typing import Any


class VariablePeriodMovingAverage:
    """Incrementally compute MAVP from values and per-bar periods."""

    def __init__(
        self,
        min_period: int = 2,
        max_period: int = 30,
        average_type: int = 0,
        _input: Any | None = None,
        periods: Any | None = None,
    ):
        """Create MAVP with optional values and per-bar periods."""
        self._state = StatefulMavp(min_period, max_period, average_type)
        if _input is not None or periods is not None:
            self.extend(_input, periods)

    def append(self, _input, period):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        period : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(_input, period)

    def extend(self, _input, periods):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        periods : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(_input, periods)

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
