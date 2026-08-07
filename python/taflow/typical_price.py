"""Canonical typical-price adapter."""

from ._native import StatefulTypprice
from ._price_state import HlcPriceState


class TypicalPrice(HlcPriceState):
    """Compute typical price through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    TypicalPrice
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulTypprice
