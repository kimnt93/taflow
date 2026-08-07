"""Canonical Accumulation/Distribution Oscillator adapter."""

from ._native import StatefulAdosc
from ._volume_state import OhlcvStateAdapter


class AccumulationDistributionOscillator(OhlcvStateAdapter):
    """Compute the Accumulation/Distribution Oscillator through Rust."""

    _native_cls = StatefulAdosc

    def __init__(
        self,
        high=None,
        low=None,
        close=None,
        volume=None,
        fastperiod: int = 3,
        slowperiod: int = 10,
    ) -> None:
        """Create the oscillator with optional initial OHLCV history."""
        super().__init__(high, low, close, volume, fastperiod, slowperiod)
