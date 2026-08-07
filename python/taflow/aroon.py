"""Native stateful Aroon indicator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulAroon
from ._series import as_float64_series


class Aroon:
    """Compute Aroon up and down values from high/low price series.

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
        """Create an Aroon state and optionally process initial prices."""
        self._state = StatefulAroon(timeperiod)
        self._values = ([], [])
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float) -> "Aroon":
        """Append one high/low bar and update the native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        value = self._state.append(float(high), float(low))
        self._values[0].append(np.nan if value is None else value[0])
        self._values[1].append(np.nan if value is None else value[1])
        return self

    def extend(self, high: Any, low: Any) -> "Aroon":
        """Append aligned high and low histories to the native state..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        values = self._state.extend(as_float64_series(high), as_float64_series(low))
        self._values[0].extend(np.asarray(values[0], dtype=np.float64).tolist())
        self._values[1].extend(np.asarray(values[1], dtype=np.float64).tolist())
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned down and up Aroon histories..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return tuple(np.asarray(values, dtype=np.float64) for values in self._values)

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest down/up pair, or ``None`` during warm-up..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "Aroon":
        """Reset native state and accumulated output history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        self._values[0].clear()
        self._values[1].clear()
        return self
