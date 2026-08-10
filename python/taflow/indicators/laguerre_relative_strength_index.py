"""Persistent Laguerre Relative Strength Index."""

from typing import Any

import numpy as np

from .._native import LaguerreRelativeStrengthIndex as _NativeLaguerreRelativeStrengthIndex
from .._series import as_float64_series


class LaguerreRelativeStrengthIndex:
    """Compute Ehlers' four-stage Laguerre oscillator on close prices.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        streaming state.
    gamma : float, default 0.5
        Laguerre smoothing coefficient in the interval ``[0, 1]``.

    Notes
    -----
    Rust initializes all four filter stages from the first close. Each later
    bar updates the stages causally and returns ``100 * upward / (upward +
    downward)``. A zero denominator produces the neutral value 50, so there is
    no scalar warm-up: a non-empty state always has a value and ``compute``
    returns one aligned value per close. Rust owns all filter arithmetic,
    state, and output history. The independent oracle/name mapping is
    ``LaguerreRelativeStrengthIndex`` to Wickra ``LaguerreRSI``. ``append``,
    ``extend``, and ``reset`` mutate and return this adapter.
    """

    def __init__(self, close: Any, gamma: float = 0.5) -> None:
        self._state = _NativeLaguerreRelativeStrengthIndex(float(gamma))
        self.extend(close)

    def append(self, close: float) -> "LaguerreRelativeStrengthIndex":
        """Append one chronological close price.

        Parameters
        ----------
        close : float
            The next close price.

        Returns
        -------
        LaguerreRelativeStrengthIndex
            This updated adapter; read ``value`` for the latest oscillator.
        """
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "LaguerreRelativeStrengthIndex":
        """Append a chronological close-price history in one native call.

        Parameters
        ----------
        close : array-like
            Numeric one-dimensional close series.

        Returns
        -------
        LaguerreRelativeStrengthIndex
            This updated adapter.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete zero-to-one-hundred oscillator history.

        Returns
        -------
        numpy.ndarray
            One oscillator value per processed close; the first value is 50.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` when empty."""
        return self._state.value

    def reset(self) -> "LaguerreRelativeStrengthIndex":
        """Restore fresh native state and clear the aligned output history.

        Returns
        -------
        LaguerreRelativeStrengthIndex
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close prices."""
        return len(self._state)


__all__ = ["LaguerreRelativeStrengthIndex"]
