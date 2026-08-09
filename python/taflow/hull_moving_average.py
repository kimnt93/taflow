"""Persistent Hull moving average."""

from typing import Any
import numpy as np
from ._native import HmaOperator as _Native
from ._series import as_float64_series


class HullMovingAverage:
    """Persistent Hull moving average.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 10,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "HullMovingAverage":
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
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "HullMovingAverage":
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
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
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

    def __len__(self) -> int:
        """Return the number of observations consumed by this state."""
        return self._length

    def reset(self) -> "HullMovingAverage":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self
