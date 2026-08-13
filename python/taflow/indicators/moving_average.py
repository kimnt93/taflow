"""Native selectable moving-average adapter."""

from typing import Any

import numpy as np

from .._native import MovingAverage as _NativeMovingAverage
from .._series import as_float64_series


class MovingAverage:
    """Compute a TA-Lib-compatible selectable moving average.

    Parameters
    ----------
    values : array-like
        Chronological input values supplied through ``extend``.
    timeperiod : int, default 30
        Lookback period forwarded to the selected native moving-average state.
    matype : int, default 0
        TA-Lib ``MA_Type`` code: 0 SMA, 1 EMA, 2 WMA, 3 DEMA, 4 TEMA,
        5 TRIMA, 6 KAMA, 7 MAMA, or 8 T3-like TripleExponentialAverage.

    Notes
    -----
    Rust owns moving-average selection, recurrence, warm-up, and output
    history. Scalar warm-up is ``None`` and aligned ``compute`` values are
    ``NaN``. ``append``, ``extend``, and ``reset`` mutate and return this
    adapter. The oracle/name mapping is ``MovingAverage`` to TA-Lib ``MA``.
    """

    def __init__(
        self,
        timeperiod: int = 30,
        matype: int = 0,
    ) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _NativeMovingAverage(timeperiod, matype)

    def append(self, value: float) -> "MovingAverage":
        """Append one value and return this adapter for fluent chaining."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "MovingAverage":
        """Append one-dimensional chronological values through Rust."""
        self._state.extend(as_float64_series(values))
        return self

    def compute(self) -> np.ndarray:
        """Return aligned selected moving-average history as float64."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest selected moving-average value, or ``None`` warm-up."""
        return self._state.value

    def reset(self) -> "MovingAverage":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed values."""
        return len(self._state)


__all__ = ["MovingAverage"]
