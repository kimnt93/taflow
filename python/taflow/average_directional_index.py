"""Descriptive stateful interface for the Average Directional Index."""

from taflow._native import StatefulAdx
from typing import Any

import numpy as np


class AverageDirectionalIndex:
    """Incrementally compute Wilder's Average Directional Index."""

    def __init__(
        self,
        period: int = 14,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Create the indicator and optionally process an initial history.

        Parameters are ``period`` (Wilder lookback), ``high``, ``low``, and
        ``close`` (aligned OHLC series).  The constructor returns no value;
        use ``extend`` for later history.
        """
        self._state = StatefulAdx(period)
        self._values: list[float] = []
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
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
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(high, low, close)
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
        self._values.clear()
        return self
