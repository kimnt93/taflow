"""Native-backed causal rolling-winsorization adapter."""

from typing import Any

import numpy as np

from ._native import RollingWinsorizeOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class RollingWinsorize:
    """Clip each latest value to trailing lower and upper quantiles.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14; ``lower`` and ``upper``
    default to 0.05 and 0.95 and must satisfy ``0 <= lower <= upper <= 1``.
    The first ``timeperiod - 1`` outputs are NaN. ``compute`` returns one
    aligned float array, ``value`` is the latest clipped value or ``None``
    during warm-up, and lifecycle mutators return ``self``. The oracle is
    pandas rolling quantile plus ``numpy.clip``.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 14,
        lower: float = 0.05,
        upper: float = 0.95,
    ) -> None:
        """Initialize native state and process the supplied input series.

        Parameters
        ----------
        _input : array-like
            Input history to process in chronological order.
        timeperiod : int, default=14
            Number of observations in the trailing quantile window.
        lower, upper : float, default=0.05, 0.95
            Inclusive lower and upper quantile bounds.

        Returns
        -------
        None
            The constructor initializes native state and returns no value.
        """
        self._state = _Native(int(timeperiod), float(lower), float(upper))
        self.extend(_input)

    def append(self, _input: float) -> "RollingWinsorize":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "RollingWinsorize":
        """Append a chronological input history and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned winsorized history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest clipped value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingWinsorize":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)


__all__ = ["RollingWinsorize"]
