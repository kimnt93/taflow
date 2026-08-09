"""Descriptive stateful interface for Parabolic SAR."""

from taflow._native import StatefulSar
from typing import Any

import numpy as np


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    ParabolicSar
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        acceleration: float = 0.02,
        maximum: float = 0.2,
    ) -> None:
        """Create Parabolic SAR with optional aligned high/low history."""
        self._state = StatefulSar(acceleration, maximum)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float) -> "ParabolicSar":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "ParabolicSar":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(high, low)
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
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "ParabolicSar":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
