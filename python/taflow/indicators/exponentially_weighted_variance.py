"""Native-backed exponentially weighted variance adapter."""

from typing import Any

import numpy as np

from .._native import ExponentiallyWeightedVarianceOperator as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class ExponentiallyWeightedVariance:
    """Compute causal exponentially weighted variance.

    ``_input`` is the required chronological numeric history and may be empty
    for a fresh stream. ``timeperiod`` defaults to 14 and controls the span of
    the pandas ``Series.ewm(..., adjust=False)`` recurrence. Rust owns all
    weighting, warm-up, and output storage; ``compute`` returns one aligned
    float array with NaN warm-up values. The independent oracle is pandas EWM
    variance with ``bias=True``. Lifecycle mutators return ``self``.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.

        Parameters
        ----------
        timeperiod : int, default 14
            Positive EWM span in bars.
        """
        self._state = _Native(int(timeperiod))

    def append(self, _input: float) -> "ExponentiallyWeightedVariance":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "ExponentiallyWeightedVariance":
        """Append one chronological input history and return this adapter."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned EWM variance history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest variance, or ``None`` during scalar warm-up."""
        return self._state.value

    def reset(self) -> "ExponentiallyWeightedVariance":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations processed by Rust."""
        return adapter_length(self)


__all__ = ["ExponentiallyWeightedVariance"]
