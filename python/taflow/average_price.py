"""Canonical average-price adapter."""

from ._native import StatefulAvgprice
from ._price_state import OhlcPriceState


class AveragePrice(OhlcPriceState):
    """Compute average price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AveragePrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAvgprice
