"""Native classic pivot-level interface."""

from typing import Any

import numpy as np

from ._native import StatefulPivotPoints


class PivotPoints:
    """Compute classic pivot, resistance, and support levels by session.

    Parameters
    ----------
    high, low, close : array-like
        Initial aligned OHLC history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        anchor: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        anchor : object
            Boolean series marking reset or anchor bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulPivotPoints()
        if close is not None:
            self.extend(high, low, close, anchor)

    def append(
        self, high: float, low: float, close: float, anchor: bool = False
    ) -> "PivotPoints":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.
        anchor : bool
            Current anchor-reset flag.

        Returns
        -------
        PivotPoints
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low), float(close), bool(anchor))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, anchor: Any | None = None
    ) -> "PivotPoints":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.
        anchor : Any | None
            Chronological anchor-reset flag series.

        Returns
        -------
        PivotPoints
            This indicator, for fluent chaining."""
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
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "PivotPoints":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        PivotPoints
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
