"""Descriptive stateful interface for Parabolic SAR."""

from taflow._native import StatefulSar
from typing import Any


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars."""

    def __init__(
        self,
        acceleration: float = 0.02,
        maximum: float = 0.2,
        high: Any | None = None,
        low: Any | None = None,
    ):
        """Create Parabolic SAR with optional aligned high/low history."""
        self._state = StatefulSar(acceleration, maximum)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high, low):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(high, low)

    def extend(self, high, low):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(high, low)

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
