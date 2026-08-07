"""Descriptive stateful interface for the Intraday Momentum Index."""

from taflow._native import StatefulImi
from typing import Any


class IntradayMomentumIndex:
    """Incrementally compare rolling intraday candle gains and losses."""

    def __init__(
        self, period: int = 14, _open: Any | None = None, close: Any | None = None
    ):
        """Create IMI with an optional aligned _open/close history."""
        self._state = StatefulImi(period)
        if _open is not None or close is not None:
            self.extend(_open, close)

    def append(self, _open, close):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _open : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(_open, close)

    def extend(self, _open, close):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _open : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(_open, close)

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
