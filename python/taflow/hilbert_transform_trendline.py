"""Descriptive stateful interface for the Hilbert Transform trendline."""

from taflow._native import StatefulHtTrendline
from typing import Any

import numpy as np


class HilbertTransformTrendline:
    """Incrementally compute the instantaneous Hilbert Transform trendline

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    HilbertTransformTrendline
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        _input: Any,
    ) -> None:
        """Create the trendline with an initial price series."""
        self._state = StatefulHtTrendline()
        if _input is not None:
            self.extend(_input)

    def append(self, _input: object) -> "HilbertTransformTrendline":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: object) -> "HilbertTransformTrendline":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series or the current scalar observation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(_input)
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

    def reset(self) -> "HilbertTransformTrendline":
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
