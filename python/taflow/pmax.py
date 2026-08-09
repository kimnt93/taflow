"""Native Parabolic Moving Average Stop interface."""

from typing import Any

import numpy as np

from ._native import StatefulParabolicMovingAverageStop


class ParabolicMovingAverageStop:
    """Compute an EMA/rolling-range stop and causal trend direction.

    Parameters
    ----------
    high, low, close : array-like
        Initial aligned OHLC history.
    length : int, default 10
        EMA and true-range lookback.
    multiplier : float, default 3
        Stop distance multiplier.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        length: int = 10,
        multiplier: float = 3.0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        multiplier : object
            Channel or volatility multiplier.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulParabolicMovingAverageStop(length, multiplier)
        if close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "ParabolicMovingAverageStop":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.

        Returns
        -------
        ParabolicMovingAverageStop
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "ParabolicMovingAverageStop":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.

        Returns
        -------
        ParabolicMovingAverageStop
            This indicator, for fluent chaining."""
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
        )
        return self

    def compute(self) -> object:
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

    def reset(self) -> "ParabolicMovingAverageStop":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        ParabolicMovingAverageStop
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
