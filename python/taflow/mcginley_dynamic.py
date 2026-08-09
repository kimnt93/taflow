"""Persistent McGinley Dynamic (pandas-ta-classic alignment)."""

from typing import Any
import numpy as np
from ._native import McGinleyDynamicOperator as _Native
from ._series import as_float64_series


class McGinleyDynamic:
    """Persistent McGinley Dynamic (pandas-ta-classic alignment).

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        close: Any,
        length: object = 10,
        c: object = 1.0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        c : object
            McGinley adjustment constant.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(length, c)
        self.extend(close) if close is not None else None

    def append(self, close: float) -> "McGinleyDynamic":
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
        self._state.append(close)
        return self

    def extend(self, close: Any) -> "McGinleyDynamic":
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
        self._state.extend(as_float64_series(close))
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

    def reset(self) -> "McGinleyDynamic":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
