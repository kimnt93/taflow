"""Pairs-trading z-score of the rolling OLS spread ``y - beta*x``."""

from typing import Any
import numpy as np
from ._native import SpreadZscoreOperator as _Native
from ._series import as_float64_series


class SpreadZscore:
    """Stateful SpreadZscore indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self, x: Any | None = None, y: Any | None = None, timeperiod: int = 20
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        x : object
            Input series, scalar parameter, or configuration value for this operation.
        y : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(timeperiod)
        if x is not None or y is not None:
            self.extend(x, y)

    def append(self, x: float, y: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        x : object
            Input series, scalar parameter, or configuration value for this operation.
        y : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(x, y)
        return self

    def extend(self, x: Any, y: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        x : object
            Input series, scalar parameter, or configuration value for this operation.
        y : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(x), as_float64_series(y))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
