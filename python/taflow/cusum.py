"""CUSUM drift-detection accumulator on a change series."""

from typing import Any
import numpy as np
from ._native import CumulativeSumControlChartOperator as _Native
from ._series import as_float64_series


class CumulativeSumControlChart:
    """Stateful CumulativeSumControlChart indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, change: Any | None = None, threshold: float = 1.0) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        change : object
            Values or parameters consumed by this operation.
        threshold : object
            Detection threshold applied to the input changes.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(threshold)
        self.extend(change) if change is not None else None

    def append(self, change: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        change : object
            Values or parameters consumed by this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(change)
        return self

    def extend(self, change: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        change : object
            Values or parameters consumed by this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(change))
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
