"""Native session volume-profile level interface."""

from typing import Any

import numpy as np

from ._native import StatefulSessionVolumeLevels


class SessionVolumeLevels:
    """Compute point of control and value-area bounds per session.

    Parameters
    ----------
    high, low, close, volume : array-like
        Initial aligned OHLCV history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    bins : int, default 24
        Number of fixed histogram price bins.
    value_area : float, default 0.7
        Fraction of session volume included in the value area.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        anchor: Any,
        bins: int = 24,
        value_area: float = 0.7,
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
        volume : object
            Volume series or the current bar volume.
        anchor : object
            Boolean series marking reset or anchor bars.
        bins : object
            Number of histogram bins.
        value_area : object
            Fraction of volume included in the value area.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulSessionVolumeLevels(bins, value_area)
        if close is not None:
            self.extend(high, low, close, volume, anchor)

    def append(
        self, high: float, low: float, close: float, volume: float, anchor: bool = False
    ) -> "SessionVolumeLevels":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.
        volume : float
            Current volume.
        anchor : bool
            Current anchor-reset flag.

        Returns
        -------
        SessionVolumeLevels
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(
            float(high), float(low), float(close), float(volume), bool(anchor)
        )
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any, anchor: Any | None = None
    ) -> "SessionVolumeLevels":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.
        volume : Any
            Chronological volume series.
        anchor : Any | None
            Chronological anchor-reset flag series.

        Returns
        -------
        SessionVolumeLevels
            This indicator, for fluent chaining."""
        close_array = np.asarray(close, dtype=np.float64)
        if anchor is None:
            anchor = np.zeros(close_array.shape, dtype=np.bool_)
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            close_array,
            np.asarray(volume, dtype=np.float64),
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

    def reset(self) -> "SessionVolumeLevels":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        SessionVolumeLevels
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
