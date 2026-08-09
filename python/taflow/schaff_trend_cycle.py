"""Persistent Schaff Trend Cycle (pandas-ta classic alignment)."""

from typing import Any
import numpy as np
from ._native import SchaffTrendCycleOperator as _Native
from ._series import as_float64_series


class SchaffTrendCycle:
    """Persistent Schaff Trend Cycle (pandas-ta classic alignment).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        close: Any,
        tclength: int = 10,
        fast: int = 12,
        slow: int = 26,
        factor: float = 0.5,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        tclength : object
            Schaff cycle length.
        fast : object
            Fast smoothing period in bars.
        slow : object
            Slow smoothing period in bars.
        factor : object
            Trend multiplier.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(tclength, fast, slow, factor)
        self._length = 0
        self.extend(close)

    def append(self, close: float) -> "SchaffTrendCycle":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(close))
        self._length += 1
        return self

    def extend(self, close: Any) -> "SchaffTrendCycle":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        close_array = as_float64_series(close)
        self._state.extend(close_array)
        self._length += len(close_array)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "SchaffTrendCycle":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed closes."""
        return self._length
