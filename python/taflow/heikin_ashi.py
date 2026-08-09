"""Native Heikin-Ashi OHLC transform interface."""

from typing import Any

import numpy as np

from ._native import StatefulHeikinAshi


class HeikinAshi:
    """Compute causal transformed _open, high, low, and close values.

    Parameters
    ----------
    _open, high, low, close : array-like
        Initial aligned OHLC history.
    """

    def __init__(
        self,
        _open: Any,
        high: Any,
        low: Any,
        close: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        _open : object
            Open-price series or the current bar open.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulHeikinAshi()
        if (
            _open is not None
            or high is not None
            or low is not None
            or close is not None
        ):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float) -> "HeikinAshi":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _open : float
            Current open price.
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.

        Returns
        -------
        HeikinAshi
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(_open), float(high), float(low), float(close))
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "HeikinAshi":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _open : Any
            Chronological open price series.
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.

        Returns
        -------
        HeikinAshi
            This indicator, for fluent chaining."""
        self._state.extend(
            np.asarray(_open, dtype=np.float64),
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
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

    def reset(self) -> "HeikinAshi":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        HeikinAshi
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
