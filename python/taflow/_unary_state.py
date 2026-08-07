"""Shared native-backed lifecycle adapter for unary indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class UnaryStateAdapter:
    """Adapt a native unary state without performing numerical work in Python

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    UnaryStateAdapter
        A persistent native-backed indicator adapter.
    """

    _native_cls = None

    def __init__(self, _input: Any | None = None, timeperiod: int = 14) -> None:
        """Create the native state and optionally process an input history."""
        if self._native_cls is None:
            raise TypeError("a native state class must be configured")
        self._state = self._native_cls(timeperiod)
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> object:
        """Append one value and update the native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        value = self._state.append(float(_input))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, _input: Any) -> object:
        """Append an aligned input history to the native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        values = self._state.extend(as_float64_series(_input))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned native output history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest native output, or ``None`` during warm-up..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Reset native state and accumulated output history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values.clear()
        return self
