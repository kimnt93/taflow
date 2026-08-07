"""Canonical Triple Exponential Moving Average adapter."""

from ._native import StatefulTema
from ._unary_state import UnaryStateAdapter


class TripleExponentialMovingAverage(UnaryStateAdapter):
    """Compute TEMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TripleExponentialMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTema
