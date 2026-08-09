"""Persistent Hilbert Transform dominant cycle period (HT_DCPERIOD)."""

from typing import Any
import numpy as np

from ._native import HilbertTransformDominantCyclePeriod as _Native
from ._series import as_float64_series


class HilbertTransformDominantCyclePeriod:
    """Persistent Hilbert Transform dominant cycle period (HT_DCPERIOD).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native()
        if _input is not None:
            self.extend(_input)

    def append(self, value: float) -> "HilbertTransformDominantCyclePeriod":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input value processed at each bar.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(value))
        return self

    def extend(self, _input: Any) -> "HilbertTransformDominantCyclePeriod":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input values processed in chronological order.

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

    def reset(self) -> "HilbertTransformDominantCyclePeriod":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
