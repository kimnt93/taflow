"""Persistent Closing CandleMarubozu candlestick recognition (CDLCLOSINGMARUBOZU)."""

from typing import Any
import numpy as np
from ._native import CandleClosingMarubozu as _Native
from ._series import as_float64_series


class CandleClosingMarubozu:
    """Persistent Closing CandleMarubozu candlestick recognition (CDLCLOSINGMARUBOZU).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_open`, `high`, `low`, `close`. Warm-up positions are represented by `NaN` in history."""

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
        self._state = _Native()
        if any(value is not None for value in (_open, high, low, close)):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float) -> "CandleClosingMarubozu":
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
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_open, high, low, close)
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "CandleClosingMarubozu":
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
        Self
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
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "CandleClosingMarubozu":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
