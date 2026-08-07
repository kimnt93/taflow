"""Canonical Chande Momentum Oscillator adapter."""

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
