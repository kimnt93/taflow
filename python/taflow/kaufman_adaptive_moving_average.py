"""Canonical Kaufman Adaptive Moving Average adapter."""

from ._native import StatefulKama
from ._unary_state import UnaryStateAdapter


class KaufmanAdaptiveMovingAverage(UnaryStateAdapter):
    """Compute KAMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    KaufmanAdaptiveMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulKama
