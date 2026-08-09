"""Persistent True Strength Index."""

from typing import Any
import numpy as np
from ._native import TrueStrengthIndexOperator as _Native
from ._series import as_float64_series


class TrueStrengthIndex:
    """Persistent True Strength Index.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
        fast: int = 13,
        slow: int = 25,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        fast : object
            Fast smoothing period in bars.
        slow : object
            Slow smoothing period in bars.
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(fast, slow)
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "TrueStrengthIndex":
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

    def extend(self, _input: Any) -> "TrueStrengthIndex":
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

    def reset(self) -> "TrueStrengthIndex":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self
