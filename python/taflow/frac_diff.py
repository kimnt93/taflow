"""Fractionally-differentiated series (AFML ch. 5, fixed-width window)."""

from typing import Any
import numpy as np
from ._native import FracDiffOperator as _Native
from ._series import as_float64_series


class FracDiff:
    """Fractionally-differentiated series (AFML ch. 5, fixed-width window).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `_input`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        _input: Any,
        d: float = 0.5,
        threshold: float = 1e-05,
    ) -> None:
        """Create fractional differencing with optional _input history.

        Parameters are ``_input`` (the aligned source series), ``d`` (the
        differencing order), and ``threshold`` (the smallest retained weight).
        """
        self._state = _Native(d, threshold)
        self.extend(_input) if _input is not None else None

    def append(self, _input: float) -> "FracDiff":
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

    def extend(self, _input: Any) -> "FracDiff":
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

    def reset(self) -> "FracDiff":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
