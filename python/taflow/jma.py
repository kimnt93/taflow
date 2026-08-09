"""Native Jurik-like adaptive moving-average interface."""

from typing import Any

import numpy as np

from ._native import StatefulJurikMovingAverage


class JurikMovingAverage:
    """Compute the documented public Jurik-like adaptive average.

    Parameters
    ----------
    close : array-like
        Initial aligned close history.
    length : int, default 7
        Lookback controlling the base adaptive coefficient.
    phase : float, default 0
        Phase parameter retained by the public reconstruction interface.
    """

    def __init__(
        self,
        close: Any,
        length: int = 7,
        phase: float = 0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        phase : object
            Hilbert transform phase parameter.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulJurikMovingAverage(length, phase)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "JurikMovingAverage":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        JurikMovingAverage
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "JurikMovingAverage":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        JurikMovingAverage
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

    def reset(self) -> "JurikMovingAverage":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        JurikMovingAverage
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
