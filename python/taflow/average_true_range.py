"""Canonical Average True Range adapter."""

from ._native import StatefulAtr
from ._ohlc_state import OhlcStateAdapter


class AverageTrueRange(OhlcStateAdapter):
    """Compute Wilder's average true range through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AverageTrueRange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAtr
