"""Canonical rolling maximum adapter."""

from ._native import StatefulMax
from ._unary_state import UnaryStateAdapter


class RollingMax(UnaryStateAdapter):
    """Compute the trailing maximum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingMax
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMax
