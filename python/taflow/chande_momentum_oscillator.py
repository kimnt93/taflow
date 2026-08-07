"""Canonical Chande Momentum Oscillator adapter."""

from ._native import StatefulCmo
from ._unary_state import UnaryStateAdapter


class ChandeMomentumOscillator(UnaryStateAdapter):
    """Compute the Chande Momentum Oscillator through Rust."""

    _native_cls = StatefulCmo
