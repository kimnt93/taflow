"""Canonical rolling sum adapter."""

from ._native import StatefulSum
from ._unary_state import UnaryStateAdapter


class RollingSum(UnaryStateAdapter):
    """Compute the trailing sum through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RollingSum
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulSum
