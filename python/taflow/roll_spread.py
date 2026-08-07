"""Roll spread estimate: ``2 * sqrt(max(0, -cov(delta_p_t, delta_p_{t-1})))``."""

from typing import Any
import numpy as np
from ._native import RollSpreadOperator as _Native
from ._series import as_float64_series


class RollSpread:
    """Stateful RollSpread indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, price: Any | None = None, timeperiod: int = 20) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self.extend(price) if price is not None else None

    def append(self, price: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(price)
        return self

    def extend(self, price: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(price))
        return self

    def compute(self) -> np.ndarray:
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
