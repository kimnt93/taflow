"""Persistent Variable Index Dynamic Average."""

from typing import Any

import numpy as np

from ._native import VariableIndexDynamicAverage as _NativeVariableIndexDynamicAverage
from ._series import as_float64_series


class VariableIndexDynamicAverage:
    """Compute a momentum-adaptive exponential average of close prices.

    Parameters
    ----------
    close : array-like
        Initial chronological close prices. Pass an empty series for a fresh
        streaming state.
    length : int, default 14
        Positive number of close changes in the rolling Chande Momentum
        Oscillator weight and prices in the initial simple-average seed.
    alpha : float or None, default None
        Exponential smoothing factor in ``(0, 1]``. ``None`` selects
        ``2 / (length + 1)``, matching pandas-ta-classic ``vidya``.

    Notes
    -----
    Rust seeds the first output after ``length`` bars with their simple mean.
    Later values apply ``alpha * abs(CMO)`` as the exponential weight, where
    CMO is computed from the latest ``length`` close changes. Scalar warm-up is
    ``None`` and ``compute`` contains NaN at the same positions. Rust owns the
    bounded rolling state, arithmetic, warm-up, and aligned output history.
    The independent oracle/name mapping is ``VariableIndexDynamicAverage`` to
    pandas-ta-classic ``vidya``. ``append``, ``extend``, and ``reset`` mutate
    and return this adapter.
    """

    def __init__(
        self,
        close: Any,
        length: int = 14,
        alpha: float | None = None,
    ) -> None:
        self._state = _NativeVariableIndexDynamicAverage(length, alpha)
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
            ``length - 1`` warm-up bars.
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
