"""Canonical percent Rate of Change Ratio adapter."""

from ._native import StatefulRocr100
from ._unary_state import UnaryStateAdapter


class RateOfChangeRatioPercent(UnaryStateAdapter):
    """Compute the 100-scaled rate-of-change ratio through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangeRatioPercent
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocr100
