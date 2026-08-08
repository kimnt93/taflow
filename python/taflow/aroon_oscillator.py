"""Native stateful Aroon oscillator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulAroonosc
from ._series import as_float64_series


class AroonOscillator:
    """Compute the Aroon oscillator from aligned high and low prices.

    Parameters
    ----------
    high, low : array-like, optional
        Initial aligned high and low histories.
    timeperiod : int, default 14
        Lookback period.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        timeperiod: int = 14,
    ) -> None:
        """Create an oscillator state and optionally process initial prices."""
        self._state = StatefulAroonosc(timeperiod)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float) -> "AroonOscillator":
        """Append one high/low bar and update the native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "AroonOscillator":
        """Append aligned high and low histories to the native state

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned oscillator history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest oscillator value, or ``None`` during warm-up

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "AroonOscillator":
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
