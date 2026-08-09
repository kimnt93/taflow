"""Canonical Accumulation/Distribution Oscillator adapter."""
from typing import Any

from ._native import StatefulAdosc
from ._volume_state import OhlcvStateAdapter


class AccumulationDistributionOscillator(OhlcvStateAdapter):
    """Compute the Accumulation/Distribution Oscillator through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AccumulationDistributionOscillator
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAdosc

    def __init__(
        self,
        high: object,
        low: object,
        close: object,
        volume: object,
        fastperiod: int = 3,
        slowperiod: int = 10,
    ) -> None:
        """Create the oscillator with initial OHLCV history."""
        super().__init__(high, low, close, volume, fastperiod, slowperiod)

    def append(self, high: float, low: float, close: float, volume: float) -> "AccumulationDistributionOscillator":
        """Append one observation and return this indicator."""
        super().append(high, low, close, volume)
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "AccumulationDistributionOscillator":
        """Append aligned histories and return this indicator."""
        super().extend(high, low, close, volume)
        return self

    def reset(self) -> "AccumulationDistributionOscillator":
        """Reset native state and return this indicator."""
        super().reset()
        return self
