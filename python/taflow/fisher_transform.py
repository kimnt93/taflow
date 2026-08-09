"""Persistent Fisher Transform."""

from typing import Any
import numpy as np
from ._native import FisherTransformOperator as _Native
from ._series import as_float64_series


class FisherTransform:
    """Compute the causal Ehlers Fisher Transform of high/low midpoints.

    Each midpoint is normalized by the trailing high and low over
    ``timeperiod`` bars, recursively smoothed, bounded, and transformed with
    the Fisher logarithm. The first ``timeperiod - 1`` outputs are ``NaN``;
    the first complete window seeds the recurrence at zero. This definition
    maps to pandas-ta-classic ``fisher(...).iloc[:, 0]`` (TA-Lib has no Fisher
    Transform function).

    Parameters
    ----------
    high, low : array-like
        Aligned initial high- and low-price histories.
    timeperiod : int, default 10
        Positive trailing normalization window in bars.

    Notes
    -----
    The object owns persistent Rust state. ``append``, ``extend``, and
    ``reset`` return this object for fluent use; ``value`` returns the latest
    scalar or ``None`` during warm-up, and ``compute`` returns the complete
    aligned NumPy history.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        timeperiod: int = 10,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high, low : array-like
            Aligned initial high- and low-price histories.
        timeperiod : int, default 10
            Positive trailing normalization window in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self.extend(high, low) if high is not None or low is not None else None

    def append(self, high: float, low: float) -> "FisherTransform":
        """Append one high/low bar to the persistent Rust state.

        Parameters
        ----------
        high, low : float
            Current bar's high and low prices.

        Returns
        -------
        FisherTransform
            This indicator, for fluent chaining. Read ``value`` for the result.
        """
        self._state.append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "FisherTransform":
        """Append aligned high/low histories to the persistent Rust state.

        Parameters
        ----------
        high, low : array-like
            Equal-length high- and low-price histories in chronological order.

        Returns
        -------
        FisherTransform
            This indicator, for fluent chaining.

        Raises
        ------
        ValueError
            If the input lengths differ.
        """
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray
            One value per processed bar, with ``NaN`` during warm-up.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float or None
            Latest transform value, or ``None`` before warm-up completes.
        """
        return self._state.value

    def reset(self) -> "FisherTransform":
        """Restore fresh-state behavior and clear the output history.

        Returns
        -------
        FisherTransform
            This indicator, for fluent chaining.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of bars processed by the Rust state."""
        return len(self._state)
