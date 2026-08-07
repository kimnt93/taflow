"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline
from typing import Any

import numpy as np


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline."""

    def __init__(self, _input: Any | None = None) -> None:
        """Create the trendline with an optional initial price series."""
        self._state = StatefulHtTrendline()
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: object) -> object:
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
        result = self._state.append(_input)
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, _input: object) -> object:
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
        result = self._state.extend(_input)
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
