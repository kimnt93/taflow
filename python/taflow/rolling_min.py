"""Canonical rolling minimum adapter."""

from ._native import StatefulMin
from ._unary_state import UnaryStateAdapter


class RollingMin(UnaryStateAdapter):
    """Compute the trailing minimum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingMin
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMin
