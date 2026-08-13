"""Shared native-backed lifecycle adapter for unary indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class UnaryStateAdapter:
    """Adapt a native unary state without performing numerical work in Python.

    The constructor accepts configuration only and creates an empty state.

    Returns
    -------
    UnaryStateAdapter
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(
        self,
        timeperiod: int = 14,
    ) -> None:
        """Create an empty native state for the configured time period."""
        if self._native_cls is None:
            raise TypeError("a native state class must be configured")
        self._state = self._native_cls(timeperiod)

    def append(self, _input: float) -> "Self":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _input : float
            Current input.

        Returns
        -------
        Self
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "Self":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _input : Any
            Chronological input series.

        Returns
        -------
        Self
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

    def reset(self) -> "Self":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        Self
            This indicator, for fluent chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
