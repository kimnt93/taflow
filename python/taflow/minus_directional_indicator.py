from typing import Any
import numpy as np
from ._native import MinusDirectionalIndicator as _Native
from ._series import as_float64_series


class MinusDirectionalIndicator:
    """Stateful MinusDirectionalIndicator indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        timeperiod: int = 14,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(timeperiod)
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, h: float, l: float, c: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        h : object
            Input series, scalar parameter, or configuration value for this operation.
        l : object
            Input series, scalar parameter, or configuration value for this operation.
        c : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(h, l, c)
        return self

    def extend(self, h: Any, l: Any | None = None, c: Any | None = None):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        h : object
            Input series, scalar parameter, or configuration value for this operation.
        l : object
            Input series, scalar parameter, or configuration value for this operation.
        c : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        if l is None or c is None:
            raise ValueError("high, low, and close must be provided together")
        self._state.extend(
            as_float64_series(h), as_float64_series(l), as_float64_series(c)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
