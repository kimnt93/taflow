"""Canonical Percentage Price Oscillator adapter."""

from .absolute_price_oscillator import AbsolutePriceOscillator
from ._native import StatefulPpo


class PercentagePriceOscillator(AbsolutePriceOscillator):
    """Compute the percentage price oscillator through Rust."""

    _native_cls = StatefulPpo

    def __init__(
        self,
        _input=None,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native PPO state and optionally process initial inputs."""
        self._state = self._native_cls(fastperiod, slowperiod, int(moving_average_type))
        self._values = []
        if _input is not None:
            self.extend(_input)
