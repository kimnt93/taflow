"""Descriptive stateful interface for Parabolic SAR."""

from taflow._native import StatefulSar
from typing import Any

import numpy as np


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars."""

    def __init__(
        self,
        acceleration: float = 0.02,
        maximum: float = 0.2,
        high: Any | None = None,
        low: Any | None = None,
    ) -> None:
        """Create Parabolic SAR with optional aligned high/low history."""
        self._state = StatefulSar(acceleration, maximum)
        self._values: list[float] = []
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(float(high), float(low))
        self._values.append(np.nan if result is None else float(result))
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(high, low)
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return aligned Parabolic SAR values."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "ParabolicSar":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
