"""Canonical Chande Momentum Oscillator adapter."""
from typing import Any

from ._native import StatefulCmo
from ._unary_state import UnaryStateAdapter


class ChandeMomentumOscillator(UnaryStateAdapter):
    """Compute the Chande Momentum Oscillator through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    ChandeMomentumOscillator
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulCmo

    def append(self, _input: float) -> "ChandeMomentumOscillator":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "ChandeMomentumOscillator":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "ChandeMomentumOscillator":
        """Reset native state and return this indicator."""
        super().reset()
        return self
