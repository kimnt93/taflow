"""Native stateful Relative Strength Index (RSI) adapter."""

from typing import Any

import numpy as np

from ._native import RelativeStrengthIndex as _NativeRelativeStrengthIndex
from ._series import as_float64_series


class RelativeStrengthIndex:
    """Compute Wilder's relative strength index over a close series.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        streaming state.
    timeperiod : int, default 14
        Wilder smoothing period, which must be at least two.

    Notes
    -----
    Rust seeds average gain and loss from the first ``timeperiod`` price
    changes, then applies Wilder's recurrence and returns an oscillator from
    zero to one hundred. Scalar output is ``None`` through the first
    ``timeperiod`` closes and ``compute`` stores NaN at those aligned
    positions. Rust owns the recurrence, warm-up, bounded state, and output
    history. The independent oracle/name mapping is
    ``RelativeStrengthIndex`` to TA-Lib ``RSI``. ``append``, ``extend``, and
    ``reset`` mutate and return this adapter.
    """

    def __init__(
        self,
        close: Any,
        timeperiod: int = 14,
    ) -> None:
        """Create an RSI state and process initial closes.

        Parameters
        ----------
        close : array-like
            Initial close-price history.
        timeperiod : int, default 14
            Wilder smoothing period.

        Returns
        -------
        None
            The instance is initialized in place.
        """
        self._state = _NativeRelativeStrengthIndex(timeperiod)
        self.extend(close)

    def append(self, close: float) -> "RelativeStrengthIndex":
        """Append one close and update the native RSI state.

        Parameters
        ----------
        close : float
            Next close price.

        Returns
        -------
        RelativeStrengthIndex
            This stateful adapter.
        """
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "RelativeStrengthIndex":
        """Append an aligned close history to the native RSI state.

        Parameters
        ----------
        close : array-like
            Close-price observations in chronological order.

        Returns
        -------
        RelativeStrengthIndex
            This stateful adapter.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned RSI history, including NaN warm-up values.

        Returns
        -------
        numpy.ndarray
            RSI values aligned to all processed close observations.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest RSI value, or ``None`` during warm-up.

        Returns
        -------
        float or None
            Latest native RSI output.
        """
        return self._state.value

    def reset(self) -> "RelativeStrengthIndex":
        """Reset the native RSI state and accumulated output history.

        Returns
        -------
        RelativeStrengthIndex
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close prices."""
        return len(self._state)


__all__ = ["RelativeStrengthIndex"]
