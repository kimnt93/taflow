"""Native-backed rolling maximum drawdown adapter."""

from typing import Any

import numpy as np

from .._native import RollingMaximumDrawdown as _Native
from .._series import as_float64_series


class RollingMaximumDrawdown:
    """Compute the deepest peak-to-trough decline in each trailing window.

    The required ``equity`` input is an aligned chronological equity or
    non-negative price series. For each full ``timeperiod`` window, the output
    is the largest ``(peak - later_value) / peak`` as a non-negative fraction;
    ``0.20`` represents a 20% decline. The default window is 14 bars, scalar
    warm-up is ``None``, and aligned warm-up positions are ``NaN``. Non-finite
    samples do not advance the native rolling state and repeat the current
    value once warm. Rust owns all arithmetic, history, and lifecycle state.

    ``append``, ``extend``, and ``reset`` mutate and return this adapter.
    ``value`` exposes the latest scalar and ``compute`` returns one float64
    array. The independent oracle is pandas ``Series.rolling.apply``; Wickra
    0.9.9 maps this class to ``MaxDrawdown``. TA-Lib has no equivalent function.

    Parameters
    ----------
    equity : object
        Required chronological equity samples. An empty series creates a fresh
        streaming state.
    timeperiod : int, default 14
        Positive trailing-window length. A value of one emits zero immediately.

    Raises
    ------
    ValueError
        If ``timeperiod`` is zero.
    """

    def __init__(self, equity: Any, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(equity)

    def append(self, equity: float) -> "RollingMaximumDrawdown":
        """Append one equity sample and return this adapter."""
        self._state.append(float(equity))
        return self

    def extend(self, equity: Any) -> "RollingMaximumDrawdown":
        """Append chronological equity samples and return this adapter."""
        self._state.extend(as_float64_series(equity))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned maximum-drawdown history as a float64 array."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest drawdown fraction, or ``None`` during warm-up."""
        return self._state.value

    @property
    def period(self) -> int:
        """Return the configured trailing-window length."""
        return self._state.period

    def reset(self) -> "RollingMaximumDrawdown":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of supplied samples, including ignored non-finite bars."""
        return len(self._state)


__all__ = ["RollingMaximumDrawdown"]
