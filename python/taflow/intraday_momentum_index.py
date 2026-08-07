"""Descriptive stateful interface for the Intraday Momentum Index."""

from taflow._native import StatefulImi
from typing import Any

import numpy as np


class IntradayMomentumIndex:
    """Incrementally compare rolling intraday candle gains and losses."""

    def __init__(
        self, period: int = 14, _open: Any | None = None, close: Any | None = None
    ) -> None:
        """Create IMI with an optional aligned _open/close history."""
        self._state = StatefulImi(period)
        self._values: list[float] = []
        if _open is not None or close is not None:
            self.extend(_open, close)

    def append(self, _open: object, close: object) -> object:
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
        result = self._state.append(_open, close)
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, _open: object, close: object) -> object:
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
        result = self._state.extend(_open, close)
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned native output history."""
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
