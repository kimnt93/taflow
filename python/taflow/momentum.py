"""Canonical Momentum adapter."""

from ._native import StatefulMom
from ._unary_state import UnaryStateAdapter


class Momentum(UnaryStateAdapter):
    """Compute price momentum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    Momentum
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMom
