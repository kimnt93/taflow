"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline
from typing import Any


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline."""

    def __init__(self, _input: Any | None = None):
        """Create the trendline with an optional initial price series."""
        self._state = StatefulHtTrendline()
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
        return self._state.append(_input)

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
        return self._state.extend(_input)

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
