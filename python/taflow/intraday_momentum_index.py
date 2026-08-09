"""Descriptive stateful interface for the Intraday Momentum Index."""

from taflow._native import StatefulImi
from typing import Any

import numpy as np


class IntradayMomentumIndex:
    """Incrementally compare rolling intraday candle gains and losses

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    IntradayMomentumIndex
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        _open: Any,
        close: Any,
        period: int = 14,
    ) -> None:
        """Create IMI with an optional aligned _open/close history."""
        self._state = StatefulImi(period)
        if _open is not None or close is not None:
            self.extend(_open, close)

    def append(self, _open: object, close: object) -> "IntradayMomentumIndex":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _open : object
            Open-price series or the current bar open.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_open, close)
        return self

    def extend(self, _open: object, close: object) -> "IntradayMomentumIndex":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _open : object
            Open-price series or the current bar open.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(_open, close)
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

    def reset(self) -> "IntradayMomentumIndex":
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
