"""Canonical Absolute Price Oscillator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulApo
from ._series import as_float64_series


class AbsolutePriceOscillator:
    """Compute the absolute price oscillator with native moving averages.

    Parameters
    ----------
    _input : array-like, optional
        Initial input history.
    fastperiod, slowperiod : int
        Fast and slow moving-average periods.
    moving_average_type : int, default 0
        Moving-average selector compatible with :class:`taflow.MaType`.
    """

    def __init__(
        self,
        _input: Any | None = None,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native APO state and optionally process initial inputs."""
        self._state = StatefulApo(fastperiod, slowperiod, int(moving_average_type))
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> object:
        """Append one input value to native APO state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> object:
        """Append an aligned input history to native APO state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return aligned APO history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest APO value

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
