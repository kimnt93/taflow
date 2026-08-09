"""Canonical Accumulation/Distribution adapter."""
from typing import Any

from ._native import StatefulAd
from ._volume_state import OhlcvStateAdapter


class AccumulationDistribution(OhlcvStateAdapter):
    """Compute cumulative Accumulation/Distribution through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AccumulationDistribution
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAd

    def append(self, high: float, low: float, close: float, volume: float) -> "AccumulationDistribution":
        """Append one observation and return this indicator."""
        super().append(high, low, close, volume)
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "AccumulationDistribution":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close, volume)
        return self

    def reset(self) -> "AccumulationDistribution":
        """Reset native state and return this indicator."""
        super().reset()
        return self
