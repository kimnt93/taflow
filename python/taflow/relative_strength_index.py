"""Native stateful Relative Strength Index (RSI) adapter."""

from typing import Any

import numpy as np

from ._native import StatefulRsi
from ._series import as_float64_series


class RelativeStrengthIndex:
    """Compute Wilder's relative strength index over a close series.

    Parameters
    ----------
    close : array-like, optional
        Initial close-price history.
    timeperiod : int, default 14
        Wilder smoothing period.
    """

    def __init__(self, close: Any | None = None, timeperiod: int = 14) -> None:
        """Create an RSI state and optionally process initial closes.

        Parameters
        ----------
        close : array-like, optional
            Initial close-price history.
        timeperiod : int, default 14
            Wilder smoothing period.

        Returns
        -------
        None
            The instance is initialized in place.
        """
        self._state = StatefulRsi(timeperiod)
        self._values: list[float] = []
        if close is not None:
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
        value = self._state.append(float(close))
        self._values.append(np.nan if value is None else value)
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
        values = self._state.extend(as_float64_series(close))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned RSI history, including NaN warm-up values.

        Returns
        -------
        numpy.ndarray
            RSI values aligned to all processed close observations.
        """
        return np.asarray(self._values, dtype=np.float64)

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
        self._values.clear()
        return self
