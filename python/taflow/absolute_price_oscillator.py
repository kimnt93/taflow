"""Canonical Absolute Price Oscillator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulApo
from ._series import as_float64_series


class AbsolutePriceOscillator:
    """Compute the absolute price oscillator with native moving averages.

    Parameters
    ----------
    _input : array-like
        Initial input history.
    fastperiod, slowperiod : int
        Fast and slow moving-average periods.
    moving_average_type : int, default 0
        Moving-average selector compatible with :class:`taflow.MaType`.
    """

    def __init__(
        self,
        _input: Any,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native APO state and process initial inputs."""
        self._state = StatefulApo(fastperiod, slowperiod, int(moving_average_type))
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "AbsolutePriceOscillator":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _input : float
            Current input.

        Returns
        -------
        AbsolutePriceOscillator
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "AbsolutePriceOscillator":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _input : Any
            Chronological input series.

        Returns
        -------
        AbsolutePriceOscillator
            This indicator, for fluent chaining."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "AbsolutePriceOscillator":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        AbsolutePriceOscillator
            This indicator, for fluent chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
