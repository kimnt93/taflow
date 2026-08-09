"""CUSUM drift-detection accumulator on a change series."""

from typing import Any
import numpy as np
from ._native import CumulativeSumControlChartOperator as _Native
from ._series import as_float64_series


class CumulativeSumControlChart:
    """CUSUM drift-detection accumulator on a change series.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `change`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        change: Any,
        threshold: float = 1.0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        change : object
            Input change series processed chronologically.
        threshold : object
            Detection threshold applied to the input changes.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(threshold)
        self.extend(change) if change is not None else None

    def append(self, change: float) -> "CumulativeSumControlChart":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        change : object
            Input change series processed chronologically.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(change)
        return self

    def extend(self, change: Any) -> "CumulativeSumControlChart":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        change : object
            Input change series processed chronologically.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(change))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "CumulativeSumControlChart":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
