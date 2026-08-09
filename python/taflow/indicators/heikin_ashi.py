"""Persistent Heikin-Ashi OHLC transform."""

from typing import Any

import numpy as np

from .._native import HeikinAshi as _NativeHeikinAshi
from .._series import as_float64_series


class HeikinAshi:
    """Compute causal Heikin-Ashi open, high, low, and close histories.

    Parameters
    ----------
    _open : array-like
        Initial chronological open prices. Pass an empty aligned series for a
        fresh streaming state.
    high : array-like
        Initial chronological high prices aligned with ``_open``.
    low : array-like
        Initial chronological low prices aligned with ``_open``.
    close : array-like
        Initial chronological close prices aligned with ``_open``.

    Notes
    -----
    The constructor requires aligned open, high, low, and close series; pass
    four empty series for a fresh streaming state. The first transformed open
    is ``(open + close) / 2``; later opens average the previous transformed
    open and close. There is no warm-up. ``compute`` returns arrays in
    ``(open, high, low, close)`` order. Rust owns the recurrence, warm-up,
    state, and output histories. The independent oracle is pandas-ta-classic
    ``ha``. ``append``, ``extend``, and ``reset`` mutate and return this adapter.
    """

    def __init__(self, _open: Any, high: Any, low: Any, close: Any) -> None:
        self._state = _NativeHeikinAshi()
        self.extend(_open, high, low, close)

    def append(
        self, _open: float, high: float, low: float, close: float
    ) -> "HeikinAshi":
        """Append one chronological OHLC bar.

        Parameters
        ----------
        _open, high, low, close : float
            The next bar in open, high, low, close order.

        Returns
        -------
        HeikinAshi
            This updated adapter; read ``value`` for the transformed bar.
        """
        self._state.append(float(_open), float(high), float(low), float(close))
        return self

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> "HeikinAshi":
        """Append aligned chronological OHLC histories.

        Parameters
        ----------
        _open, high, low, close : array-like
            Series in open, high, low, close order. Unequal lengths are
            rejected before the native state or histories are mutated.

        Returns
        -------
        HeikinAshi
            This updated adapter.
        """
        self._state.extend(
            as_float64_series(_open),
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        """Return the complete transformed histories.

        Returns
        -------
        tuple of numpy.ndarray
            Four arrays in open, high, low, close order. Every input bar has
            an output because this transform has no warm-up period.
        """
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float, float] | None:
        """Return the latest open/high/low/close tuple, or ``None`` when empty."""
        return self._state.value

    def reset(self) -> "HeikinAshi":
        """Restore fresh native state and clear all four output histories.

        Returns
        -------
        HeikinAshi
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed OHLC bars."""
        return len(self._state)


__all__ = ["HeikinAshi"]
