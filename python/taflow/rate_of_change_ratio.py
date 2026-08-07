"""Canonical Rate of Change Ratio adapter."""

from ._native import StatefulRocr
from ._unary_state import UnaryStateAdapter


class RateOfChangeRatio(UnaryStateAdapter):
    """Compute the rate-of-change ratio through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangeRatio
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocr
