"""Persistent On Neck recognition (CDLONNECK)."""

from typing import Any
import numpy as np
from .._native import CandleOnNeck as _Native
from .._candle_ohlc import as_ohlc_arrays


class CandleOnNeck:
    """Persistent On Neck recognition (CDLONNECK).

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
        (
            self.extend(_open, high, low, close)
            if any(x is not None for x in (_open, high, low, close))
            else None
        )

    def append(self, _open: float, high: float, low: float, close: float) -> "CandleOnNeck":
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
        self._state.append(float(_open), float(high), float(low), float(close))
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "CandleOnNeck":
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
        self._state.extend(*as_ohlc_arrays(_open, high, low, close))
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
    def value(self) -> int | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def __len__(self) -> int:
        """Return the number of processed OHLC bars."""
        return len(self._state.compute())

    def reset(self) -> "CandleOnNeck":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
