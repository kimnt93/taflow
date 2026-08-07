"""Canonical Accumulation/Distribution Oscillator adapter."""

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
        high: object = None,
        low: object = None,
        close: object = None,
        volume: object = None,
        fastperiod: int = 3,
        slowperiod: int = 10,
    ) -> None:
        """Create the oscillator with optional initial OHLCV history."""
        super().__init__(high, low, close, volume, fastperiod, slowperiod)
