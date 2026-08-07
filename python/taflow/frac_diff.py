"""Fractionally-differentiated series (AFML ch. 5, fixed-width window)."""

from typing import Any
import numpy as np
from ._native import FracDiffOperator as _Native
from ._series import as_float64_series


class FracDiff:
    """Stateful FracDiff indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self, _input: Any | None = None, d: float = 0.5, threshold: float = 1e-5
    ) -> None:
        """Create fractional differencing with optional _input history.

        Parameters are ``_input`` (the aligned source series), ``d`` (the
        differencing order), and ``threshold`` (the smallest retained weight).
        """
        self._state = _Native(d, threshold)
        self.extend(_input) if _input is not None else None

    def append(self, _input: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input))
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
