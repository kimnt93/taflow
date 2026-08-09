"""Pairs-trading z-score of the rolling OLS spread ``y - beta*x``."""

from typing import Any
import numpy as np
from ._native import SpreadZScoreOperator as _Native
from ._series import as_float64_series


class SpreadZScore:
    """Pairs-trading z-score of the rolling OLS spread ``y - beta*x``.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `x`, `y`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        x: Any,
        y: Any,
        timeperiod: int = 20,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(x, y)

    def append(self, x: float, y: float) -> "SpreadZScore":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(x), float(y))
        self._length += 1
        return self

    def extend(self, x: Any, y: Any) -> "SpreadZScore":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        x : object
            First aligned input series or scalar observation.
        y : object
            Second aligned input series or scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        x_array = as_float64_series(x)
        y_array = as_float64_series(y)
        if x_array.shape != y_array.shape:
            raise ValueError("x and y must have equal lengths")
        self._state.extend(x_array, y_array)
        self._length += len(x_array)
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

    def reset(self) -> "SpreadZScore":
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
        """Return the number of processed pairs."""
        return self._length
