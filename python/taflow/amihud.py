"""Amihud illiquidity: rolling mean of ``|ret| / (close * volume)``."""

from typing import Any
import numpy as np
from ._native import AmihudOperator as _Native
from ._series import as_float64_series


class Amihud:
    """Stateful Amihud indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self, close: Any | None = None, volume: Any | None = None, timeperiod: int = 20
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        (
            self.extend(close, volume)
            if any(value is not None for value in (close, volume))
            else None
        )

    def append(self, close: float, volume: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(close, volume)
        return self

    def extend(self, close: Any, volume: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(close), as_float64_series(volume))
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
