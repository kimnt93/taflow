"""Persistent Ultimate Oscillator."""

from typing import Any
import numpy as np
from ._native import UltimateOscillator as _Native
from ._series import as_float64_series


class UltimateOscillator:
    """Persistent Ultimate Oscillator.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        timeperiod1: int = 7,
        timeperiod2: int = 14,
        timeperiod3: int = 28,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        timeperiod1 : object
            First trailing period.
        timeperiod2 : object
            Second trailing period.
        timeperiod3 : object
            Third trailing period.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod1, timeperiod2, timeperiod3)
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "UltimateOscillator":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close)
        return self

    def extend(
        self, high: Any, low: Any | None = None, close: Any | None = None
    ) -> "UltimateOscillator":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        if low is None or close is None:
            raise ValueError("high, low, and close must be provided together")
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "UltimateOscillator":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Execute the __len__ operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return len(self._state)
