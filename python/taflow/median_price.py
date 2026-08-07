"""Canonical median-price adapter."""

from ._native import StatefulMedprice
from ._price_state import HlPriceState


class MedianPrice(HlPriceState):
    """Compute median price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MedianPrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulMedprice
