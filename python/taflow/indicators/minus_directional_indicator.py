from typing import Any
import numpy as np
from .._native import MinusDirectionalIndicator as _Native
from .._series import as_float64_series


class MinusDirectionalIndicator:
    """Minus Directional Indicator

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        timeperiod: int = 14,
    ) -> None:
        """Initialize an empty configured native state.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)

    def append(self, h: float, l: float, c: float) -> "MinusDirectionalIndicator":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        h : object
            Input parameter or configuration value for this operation.
        l : object
            Input parameter or configuration value for this operation.
        c : object
            Input parameter or configuration value for this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(h, l, c)
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "MinusDirectionalIndicator":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            Input parameter or configuration value for this operation.
        low : object
            Input parameter or configuration value for this operation.
        close : object
            Input parameter or configuration value for this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
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

    def reset(self) -> "MinusDirectionalIndicator":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
