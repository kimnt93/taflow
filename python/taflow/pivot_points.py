"""Native classic pivot-level interface."""

from typing import Any

import numpy as np

from ._native import StatefulPivotPoints


class PivotPoints:
    """Compute classic pivot, resistance, and support levels by session.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned OHLC history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        anchor: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        anchor : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulPivotPoints()
        if close is not None:
            self.extend(high, low, close, anchor)

    def append(
        self, high: float, low: float, close: float, anchor: bool = False
    ) -> object:
        """Process one OHLC bar and return five pivot levels."""
        return self._state.append(float(high), float(low), float(close), bool(anchor))

    def extend(
        self, high: Any, low: Any, close: Any, anchor: Any | None = None
    ) -> object:
        """Process aligned OHLC history and return this indicator."""
        close_array = np.asarray(close, dtype=np.float64)
        if anchor is None:
            anchor = np.zeros(close_array.shape, dtype=np.bool_)
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            close_array,
            np.asarray(anchor, dtype=np.bool_),
        )
        return self

    def compute(self) -> object:
        """Return pivot, resistance, and support level histories."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest five pivot levels."""
        return self._state.value

    def reset(self) -> object:
        """Clear session extrema and pivot output."""
        self._state.reset()
        return self
