"""Native Relative Momentum Index interface."""

from typing import Any

import numpy as np

from ._native import StatefulRelativeMomentumIndex


class RelativeMomentumIndex:
    """Compute Wilder-smoothed momentum gains over an aligned price series.

    Parameters
    ----------
    close : array-like
        Initial price history. Values are processed in input order.
    length : int, default 14
        Number of momentum observations used for Wilder smoothing.
    mom : int, default 5
        Lag, in bars, used to measure each momentum change.
    """

    def __init__(
        self,
        close: Any,
        length: int = 14,
        mom: int = 5,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        mom : object
            Momentum lookback in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulRelativeMomentumIndex(length, mom)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "RelativeMomentumIndex":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        RelativeMomentumIndex
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "RelativeMomentumIndex":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        RelativeMomentumIndex
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "RelativeMomentumIndex":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        RelativeMomentumIndex
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
