"""Native Relative Momentum Index adapter."""

from typing import Any

import numpy as np

from .._native import RelativeMomentumIndex as _NativeRelativeMomentumIndex
from .._series import as_float64_series


class RelativeMomentumIndex:
    """Compute Wilder-smoothed relative momentum over a close series.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        state intended for later ``append`` or ``extend`` calls.
    timeperiod : int, default 14
        Number of momentum observations used to seed and smooth average gains
        and losses. Must be at least one.
    momentum : int, default 5
        Number of bars between each close and its comparison close. Must be at
        least one.

    Notes
    -----
    The Rust state computes ``change = close[t] - close[t-momentum]``, seeds
    Wilder average gain/loss after ``timeperiod`` changes, and then applies
    Wilder's recurrence. Scalar output is ``None`` through the first
    ``momentum + timeperiod`` closes; ``compute`` aligns those warm-up values
    as ``NaN`` and returns one ``float64`` value per input close. Flat windows
    return 50.0. The independent oracle/name mapping is
    ``RelativeMomentumIndex`` to Wickra ``RMI`` (version 0.9.9). ``append``,
    ``extend``, and ``reset`` mutate and return this adapter.
    """

    def __init__(
        self,
        close: Any,
        timeperiod: int = 14,
        momentum: int = 5,
    ) -> None:
        """Create an RMI state and process initial close prices.

        Parameters
        ----------
        close : array-like
            Initial chronological close-price history; empty is allowed.
        timeperiod : int, default 14
            Wilder smoothing period.
        momentum : int, default 5
            Momentum comparison lag in bars.
        """
        self._state = _NativeRelativeMomentumIndex(timeperiod, momentum)
        self.extend(close)

    def append(self, close: float) -> "RelativeMomentumIndex":
        """Append one close and update the native RMI state.

        Parameters
        ----------
        close : float
            Next chronological close price.

        Returns
        -------
        RelativeMomentumIndex
            This stateful adapter for fluent chaining; read ``value`` for the
            latest scalar result.
        """
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "RelativeMomentumIndex":
        """Append a chronological close history to the native RMI state.

        Parameters
        ----------
        close : array-like
            One-dimensional close-price series. Values are converted once at
            the native boundary; no indicator arithmetic runs in Python.

        Returns
        -------
        RelativeMomentumIndex
            This stateful adapter for fluent chaining.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return aligned RMI history, including NaN warm-up positions.

        Returns
        -------
        numpy.ndarray
            Float64 RMI values aligned one-for-one with ``close``.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest RMI value, or ``None`` during scalar warm-up."""
        return self._state.value

    def reset(self) -> "RelativeMomentumIndex":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close prices."""
        return len(self._state)


__all__ = ["RelativeMomentumIndex"]
