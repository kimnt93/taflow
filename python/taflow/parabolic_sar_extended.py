"""Descriptive stateful interface for extended Parabolic SAR."""

from taflow._native import StatefulSarext
from typing import Any


class ParabolicSarExtended:
    """Incrementally compute signed SAREXT with independent trend settings."""

    def __init__(
        self,
        start_value=0.0,
        offset_on_reverse=0.0,
        acceleration_init_long=0.02,
        acceleration_long=0.02,
        acceleration_max_long=0.2,
        acceleration_init_short=0.02,
        acceleration_short=0.02,
        acceleration_max_short=0.2,
        high: Any | None = None,
        low: Any | None = None,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        start_value : object
            Input series, scalar parameter, or configuration value for this operation.
        offset_on_reverse : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_init_long : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_long : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_max_long : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_init_short : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_short : object
            Input series, scalar parameter, or configuration value for this operation.
        acceleration_max_short : object
            Input series, scalar parameter, or configuration value for this operation.
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulSarext(
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
        )
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
