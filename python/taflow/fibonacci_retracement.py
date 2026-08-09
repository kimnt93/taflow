"""Persistent rolling Fibonacci retracement levels."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import FibonacciRetracement as _NativeFibonacciRetracement
from ._series import as_float64_series


class FibonacciRetracement:
    """Compute seven causal levels across a rolling close-price range.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        streaming state.
    window : int, default 120
        Positive trailing window used for the rolling high and low.

    Notes
    -----
    For ratios ``(0, 0.236, 0.382, 0.5, 0.618, 0.786, 1)``, each output is
    ``rolling_high - (rolling_high - rolling_low) * ratio``. There is no
    warm-up: the window uses all available observations from the first bar.
    ``compute`` and ``value`` return levels in that ratio order. Rust owns the
    monotonic rolling state and all output calculations. The independent
    oracle is pandas ``Rolling.min``/``Rolling.max`` with ``min_periods=1``.
    ``append``, ``extend``, and ``reset`` mutate and return this adapter.
    """

    def __init__(self, close: Any, window: int = 120) -> None:
        """Create validated native state and process the required close series."""
        self._state = _NativeFibonacciRetracement(window)
        self.extend(close)

    def append(self, close: float) -> "FibonacciRetracement":
        """Append one chronological close and return this indicator.

        Parameters
        ----------
        close : float
            The next close price.

        Returns
        -------
        FibonacciRetracement
            This updated adapter; read ``value`` for the seven latest levels.
        """
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "FibonacciRetracement":
        """Append a chronological close-price series and return this indicator.

        Parameters
        ----------
        close : array-like
            Close prices converted once at the native boundary.

        Returns
        -------
        FibonacciRetracement
            This updated adapter.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(
        self,
    ) -> tuple[
        np.ndarray,
        np.ndarray,
        np.ndarray,
        np.ndarray,
        np.ndarray,
        np.ndarray,
        np.ndarray,
    ]:
        """Return all seven aligned level histories from zero to 100 percent."""
        return self._state.compute()

    @property
    def value(
        self,
    ) -> tuple[float, float, float, float, float, float, float] | None:
        """Return the latest seven levels, or ``None`` when the state is empty."""
        return self._state.value

    def reset(self) -> "FibonacciRetracement":
        """Clear rolling state and output histories while retaining the window.

        Returns
        -------
        FibonacciRetracement
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close prices."""
        return len(self._state)


__all__ = ["FibonacciRetracement"]
