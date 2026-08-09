"""Native stateful Aroon indicator adapter."""

from typing import Any

import numpy as np

from ._native import StatefulAroon
from ._series import as_float64_series


class Aroon:
    """Compute Aroon up and down values from high/low price series.

    Parameters
    ----------
    high, low : array-like
        Initial aligned high and low histories.
    timeperiod : int, default 14
        Lookback period.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        timeperiod: int = 14,
    ) -> None:
        """Create an Aroon state and process initial prices."""
        self._state = StatefulAroon(timeperiod)
        if high is not None or low is not None:
            self.extend(high, low)

    def append(self, high: float, low: float) -> "Aroon":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.

        Returns
        -------
        Aroon
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "Aroon":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.

        Returns
        -------
        Aroon
            This indicator, for fluent chaining."""
        self._state.extend(as_float64_series(high), as_float64_series(low))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "Aroon":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        Aroon
            This indicator, for fluent chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
