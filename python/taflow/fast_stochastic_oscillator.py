"""Descriptive stateful interface for the Fast Stochastic Oscillator."""

from taflow._native import StatefulStochf
from typing import Any

import numpy as np


class FastStochasticOscillator:
    """Incrementally compute aligned fast %K and fast %D

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    FastStochasticOscillator
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        fast_k_period: int = 5,
        fast_d_period: int = 3,
        fast_d_average_type: int = 0,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Create fast stochastic with optional aligned OHLC history."""
        self._state = StatefulStochf(
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )
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
        self._state.append(high, low, close)
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
        self._state.extend(high, low, close)
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the aligned native output histories

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
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

    def reset(self) -> object:
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
