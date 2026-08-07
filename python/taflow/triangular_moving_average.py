"""Canonical Triangular Moving Average adapter."""

from ._native import StatefulTrima
from ._unary_state import UnaryStateAdapter


class TriangularMovingAverage(UnaryStateAdapter):
    """Compute the triangular moving average through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TriangularMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTrima
