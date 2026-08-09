"""Canonical Percentage Price Oscillator adapter."""
from typing import Any

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
        _input: object,
        fastperiod: int = 12,
        slowperiod: int = 26,
        moving_average_type: int = 0,
    ) -> None:
        """Create native PPO state and process initial inputs."""
        self._state = self._native_cls(fastperiod, slowperiod, int(moving_average_type))
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "PercentagePriceOscillator":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "PercentagePriceOscillator":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "PercentagePriceOscillator":
        """Reset native state and return this indicator."""
        super().reset()
        return self
