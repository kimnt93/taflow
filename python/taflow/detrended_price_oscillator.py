"""Persistent causal Detrended Price Oscillator."""

from typing import Any

import numpy as np

from ._native import DetrendedPriceOscillatorOperator as _Native
from ._series import as_float64_series


class DetrendedPriceOscillator:
    """Causal DPO; pandas-ta centered/lookahead output is not exposed

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    DetrendedPriceOscillator
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        close: Any,
        period: int = 20,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        period : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(period)
        self._length = 0
        self.extend(close)

    def append(self, close: float) -> "DetrendedPriceOscillator":
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
        self._state.append(float(close))
        self._length += 1
        return self

    def extend(self, close: Any) -> "DetrendedPriceOscillator":
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
        close_array = as_float64_series(close)
        self._state.extend(close_array)
        self._length += len(close_array)
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
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "DetrendedPriceOscillator":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed closes."""
        return self._length
