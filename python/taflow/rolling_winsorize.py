"""Persistent causal rolling winsorization operator."""

from typing import Any
import numpy as np
from ._native import RollingWinsorizeOperator as _Native
from ._series import as_float64_series


class RollingWinsorize:
    """Persistent causal rolling winsorization operator.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 14,
        lower: float = 0.05,
        upper: float = 0.95,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.
        lower : object
            Lower clipping or quantile bound.
        upper : object
            Upper clipping or quantile bound.
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod, lower, upper)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "RollingWinsorize":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> "RollingWinsorize":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input))
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

    def reset(self) -> "RollingWinsorize":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
