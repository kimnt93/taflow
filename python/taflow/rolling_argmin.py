"""Canonical rolling argmin adapter."""

from ._native import StatefulMinindex
from ._unary_state import UnaryStateAdapter


class RollingArgmin(UnaryStateAdapter):
    """Compute trailing minimum indices through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingArgmin
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMinindex
