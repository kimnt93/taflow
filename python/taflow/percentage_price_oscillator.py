"""Canonical Percentage Price Oscillator adapter."""

from .absolute_price_oscillator import AbsolutePriceOscillator
from ._native import StatefulPpo


class PercentagePriceOscillator(AbsolutePriceOscillator):
    """Compute the percentage price oscillator through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    PercentagePriceOscillator
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulPpo

    def __init__(
        self,
        _input: object = None,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native PPO state and optionally process initial inputs."""
        self._state = self._native_cls(fastperiod, slowperiod, int(moving_average_type))
        if _input is not None:
            self.extend(_input)
