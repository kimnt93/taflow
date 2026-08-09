"""Causal WorldQuant time-series rank."""
from typing import Any

from .rolling_rank import RollingRank


class TimeSeriesRank(RollingRank):
    """Rank the current value within its trailing window as a fraction.

    This is the WorldQuant name for the native causal ``RollingRank`` kernel.
    """

    def append(self, _input: float) -> "TimeSeriesRank":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "TimeSeriesRank":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "TimeSeriesRank":
        """Reset native state and return this indicator."""
        super().reset()
        return self
