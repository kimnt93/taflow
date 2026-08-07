"""Causal WorldQuant time-series rank."""

from .rolling_rank import RollingRank


class TimeSeriesRank(RollingRank):
    """Rank the current value within its trailing window as a fraction.

    This is the WorldQuant name for the native causal ``RollingRank`` kernel.
    """
