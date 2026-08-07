"""Persistent Dragonfly CandleDoji candlestick recognition (CDLDRAGONFLYDOJI)."""

from typing import Any
import numpy as np
from ._native import CandleDragonflyDoji as _Native
from ._series import as_float64_series


class CandleDragonflyDoji:
    """Stateful CandleDragonflyDoji indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        _open: Any | None = None,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

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
        self._state = _Native()
        (
            self.extend(_open, high, low, close)
            if any(value is not None for value in (_open, high, low, close))
            else None
        )

    def append(self, _open: float, high: float, low: float, close: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_open, high, low, close)
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> object:
        """Append aligned input series to the native Rust state.

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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            as_float64_series(_open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
