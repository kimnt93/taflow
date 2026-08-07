"""Persistent Know Sure Thing (bukosabino `ta` alignment)."""

from typing import Any
import numpy as np
from ._native import KstOperator as _Native
from ._series import as_float64_series


class KnowSureThing:
    """Stateful KnowSureThing indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        close: Any | None = None,
        roc1: int = 10,
        roc2: int = 15,
        roc3: int = 20,
        roc4: int = 30,
        sma1: int = 10,
        sma2: int = 10,
        sma3: int = 10,
        sma4: int = 15,
        signal: int = 9,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        roc1 : object
            Input series, scalar parameter, or configuration value for this operation.
        roc2 : object
            Input series, scalar parameter, or configuration value for this operation.
        roc3 : object
            Input series, scalar parameter, or configuration value for this operation.
        roc4 : object
            Input series, scalar parameter, or configuration value for this operation.
        sma1 : object
            Input series, scalar parameter, or configuration value for this operation.
        sma2 : object
            Input series, scalar parameter, or configuration value for this operation.
        sma3 : object
            Input series, scalar parameter, or configuration value for this operation.
        sma4 : object
            Input series, scalar parameter, or configuration value for this operation.
        signal : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, signal)
        self.extend(close) if close is not None else None

    def append(self, close: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(close)
        return self

    def extend(self, close: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
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
