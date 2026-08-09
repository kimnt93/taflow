"""Canonical On-Balance Volume adapter."""
from typing import Any

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

    def append(self, close: float, volume: float) -> "OnBalanceVolume":
        """Append one observation and return this indicator."""
        super().append(close, volume)
        return self

    def extend(self, close: Any, volume: Any) -> "OnBalanceVolume":
        """Append aligned histories and return this indicator."""
        super().extend(close, volume)
        return self

    def reset(self) -> "OnBalanceVolume":
        """Reset native state and return this indicator."""
        super().reset()
        return self
