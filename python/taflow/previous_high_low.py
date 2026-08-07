"""Causal prior-higher-timeframe high/low tracking with break flags."""

from typing import Any
import numpy as np
from ._native import PreviousHighLowOperator as _Native
from ._series import as_float64_series


class PreviousHighLow:
    """Stateful PreviousHighLow indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        new_session: Any | None = None,
        high: Any | None = None,
        low: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        new_session : object
            Input series, scalar parameter, or configuration value for this operation.
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native()
        (
            self.extend(new_session, high, low)
            if new_session is not None or high is not None or low is not None
            else None
        )

    def append(self, new_session: bool, high: float, low: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        new_session : object
            Input series, scalar parameter, or configuration value for this operation.
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(new_session, high, low)
        return self

    def extend(self, new_session: Any, high: Any, low: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        new_session : object
            Input series, scalar parameter, or configuration value for this operation.
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            np.asarray(new_session, dtype=bool),
            as_float64_series(high),
            as_float64_series(low),
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
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
