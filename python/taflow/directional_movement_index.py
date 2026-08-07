"""Descriptive stateful interface for the Directional Movement Index."""

from taflow._native import StatefulDx
from typing import Any

import numpy as np


class DirectionalMovementIndex:
    """Incrementally compute Wilder's Directional Movement Index

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    DirectionalMovementIndex
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        period: int = 14,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Create DX with an optional aligned high/low/close history."""
        self._state = StatefulDx(period)
        self._values: list[float] = []
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(high, low, close)
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, high: object, low: object, close: object) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(high, low, close)
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned native output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
