"""Native-backed causal rolling-percentile adapter."""

from typing import Any

import numpy as np

from ._native import RollingPercentileOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class RollingPercentile:
    """Compute a trailing percentile on a 0--100 scale.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14 and ``percentile`` to 50.0;
    the latter must lie in ``[0, 100]``. Rust owns sorted-window warm-up and
    interpolation. ``compute`` returns one aligned float array, ``value`` is
    the latest scalar or ``None`` while warming up, and all lifecycle mutators
    return ``self``. The independent oracle is pandas rolling quantile.
    """

    def __init__(
        self, _input: Any, timeperiod: int = 14, percentile: float = 50.0
    ) -> None:
        self._state = _Native(int(timeperiod), float(percentile))
        self.extend(_input)

    def append(self, _input: float) -> "RollingPercentile":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "RollingPercentile":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned trailing-percentile history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest percentile, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingPercentile":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return adapter_length(self)


__all__ = ["RollingPercentile"]
