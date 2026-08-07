"""Canonical On-Balance Volume adapter."""

from ._native import StatefulObv
from ._volume_state import CloseVolumeStateAdapter


class OnBalanceVolume(CloseVolumeStateAdapter):
    """Compute On-Balance Volume through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    OnBalanceVolume
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulObv
