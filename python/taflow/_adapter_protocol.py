"""Shared lifecycle helpers for native-backed public adapters."""

from __future__ import annotations

from collections.abc import Mapping, Sized
from typing import Any


def adapter_length(adapter: Any) -> int:
    """Return the number of observations retained by an adapter's Rust state.

    Current native states expose ``__len__`` directly.  The history fallback
    keeps older extension classes protocol-compatible while those bindings are
    migrated; it still asks Rust for the already-computed history and performs
    no indicator calculation in Python.
    """
    state = adapter._state
    if isinstance(state, Sized):
        return len(state)

    history = state.compute()
    if isinstance(history, Mapping):
        history = next(iter(history.values()), ())
    elif isinstance(history, tuple):
        history = history[0] if history else ()
    return len(history)


def install_adapter_protocol(namespace: dict[str, Any], public_names: list[str]) -> None:
    """Fill lifecycle methods shared by every native-backed public adapter."""
    for name in public_names:
        cls = namespace.get(name)
        if (
            isinstance(cls, type)
            and hasattr(cls, "append")
            and hasattr(cls, "compute")
            and not hasattr(cls, "__len__")
        ):
            cls.__len__ = adapter_length
