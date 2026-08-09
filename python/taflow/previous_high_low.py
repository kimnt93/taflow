"""Causal prior-higher-timeframe high/low tracking with break flags."""

from typing import Any
import numpy as np
from ._native import PreviousHighLowOperator as _Native
from ._series import as_float64_series


class PreviousHighLow:
    """Causal prior-higher-timeframe high/low tracking with break flags.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `new_session`, `high`, `low`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        new_session: Any,
        high: Any,
        low: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        new_session : object
            Boolean series marking the start of each session.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native()
        (
            self.extend(new_session, high, low)
            if new_session is not None or high is not None or low is not None
            else None
        )

    def append(self, new_session: bool, high: float, low: float) -> "PreviousHighLow":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        new_session : object
            Boolean series marking the start of each session.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(new_session, high, low)
        return self

    def extend(self, new_session: Any, high: Any, low: Any) -> "PreviousHighLow":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        new_session : object
            Boolean series marking the start of each session.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            np.asarray(new_session, dtype=bool),
            as_float64_series(high),
            as_float64_series(low),
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
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

    def reset(self) -> "PreviousHighLow":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
