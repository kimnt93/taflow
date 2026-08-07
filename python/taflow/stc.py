"""Persistent Schaff Trend Cycle (pandas-ta classic alignment)."""

from typing import Any
import numpy as np
from ._native import SchaffTrendCycleOperator as _Native
from ._series import as_float64_series


class SchaffTrendCycle:
    """Stateful SchaffTrendCycle indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        close: Any | None = None,
        tclength: int = 10,
        fast: int = 12,
        slow: int = 26,
        factor: float = 0.5,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        tclength : object
            Schaff cycle length.
        fast : object
            Fast smoothing period in bars.
        slow : object
            Slow smoothing period in bars.
        factor : object
            Trend multiplier.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(tclength, fast, slow, factor)
        self.extend(close) if close is not None else None

    def append(self, close: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(close)
        return self

    def extend(self, close: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
