"""Native premium/discount zone interface."""

from typing import Any

import numpy as np

from ._native import StatefulPremiumDiscount


class PremiumDiscount:
    """Classify closes against the midpoint of a rolling swing range.

    Parameters
    ----------
    close : array-like
        Initial aligned close history.
    window : int, default 20
        Number of closes used to calculate the rolling high and low.
    """

    def __init__(
        self,
        close: Any,
        window: int = 20,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        window : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulPremiumDiscount(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "PremiumDiscount":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        PremiumDiscount
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "PremiumDiscount":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        PremiumDiscount
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(close, dtype=np.float64))
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

    def reset(self) -> "PremiumDiscount":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        PremiumDiscount
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
