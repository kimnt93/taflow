"""Persistent Variable Index Dynamic Average."""

from typing import Any

import numpy as np

from .._native import VariableIndexDynamicAverage as _NativeVariableIndexDynamicAverage
from .._series import as_float64_series


class VariableIndexDynamicAverage:
    """Compute a momentum-adaptive exponential average of close prices.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        streaming state.
    length : int, default 14
        EMA-equivalent period used to derive ``2 / (length + 1)``.
    cmo_period : int, default 9
        Number of close changes in the rolling Chande Momentum Oscillator.
    alpha : float or None, default None
        Exponential smoothing factor in ``(0, 1]``. ``None`` selects
        ``2 / (length + 1)``, matching Wickra ``VIDYA``.

    Notes
    -----
    The first output is the close that completes the CMO warm-up. Later values
    apply ``alpha * abs(CMO) / 100`` as the exponential weight. Scalar warm-up
    is ``None`` and aligned history contains NaN. Rust owns all bounded rolling
    state and arithmetic. The independent oracle/name mapping is
    ``VariableIndexDynamicAverage`` to Wickra ``VIDYA``.
    """

    def __init__(
        self,
        close: Any,
        length: int = 14,
        cmo_period: int = 9,
        alpha: float | None = None,
    ) -> None:
        """Initialize and process a chronological close-price history."""
        self._state = _NativeVariableIndexDynamicAverage(length, cmo_period, alpha)
        self.extend(close)

    def append(self, close: float) -> "VariableIndexDynamicAverage":
        """Append one chronological close price.

        Parameters
        ----------
        close : float
            The next close price.

        Returns
        -------
        VariableIndexDynamicAverage
            This updated adapter; read ``value`` for the latest output.
        """
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "VariableIndexDynamicAverage":
        """Append a chronological close-price history in one native call.

        Parameters
        ----------
        close : array-like
            Numeric one-dimensional close series.

        Returns
        -------
        VariableIndexDynamicAverage
            This updated adapter.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned average history.

        Returns
        -------
        numpy.ndarray
            One value per processed close, with NaN during the first
            ``cmo_period`` warm-up bars.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest average, or ``None`` during warm-up or when empty."""
        return self._state.value

    def reset(self) -> "VariableIndexDynamicAverage":
        """Restore fresh native state and clear the aligned output history.

        Returns
        -------
        VariableIndexDynamicAverage
            This reset adapter.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close prices."""
        return len(self._state)


__all__ = ["VariableIndexDynamicAverage"]
