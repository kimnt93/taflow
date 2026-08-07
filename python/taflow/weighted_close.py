"""Canonical weighted-close adapter."""

from ._native import StatefulWclprice
from ._price_state import HlcPriceState


class WeightedClose(HlcPriceState):
    """Compute weighted close through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    WeightedClose
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulWclprice
