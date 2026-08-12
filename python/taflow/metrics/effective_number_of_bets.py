"""Entropy-effective count of independent portfolio risk bets."""

from __future__ import annotations

from typing import Any

import numpy as np

from .._native.metrics import EffectiveNumberOfBets as _Native
from ._input import as_metric_series


class EffectiveNumberOfBets:
    """Compute the exponential entropy of independent risk contributions.

    The preferred ``from_weights_and_covariance`` input method diagonalizes a
    symmetric positive-semidefinite covariance matrix, projects portfolio
    weights into its orthogonal principal-component factors, and forms
    contributions ``eigenvalue[i] * exposure[i]**2``. After normalization to
    probabilities ``p``, the metric is ``exp(-sum(p * log(p)))``. This is the
    PCA diversification-distribution definition associated with Meucci's
    effective number of bets; NumPy ``eigh`` is the executable oracle.

    ``from_risk_contributions`` accepts an already independent non-negative
    distribution for streaming use. Rust then maintains O(1) append and
    compute state using the unnormalized entropy identity. Warm-up requires at
    least one positive total contribution; an empty or all-zero distribution
    is ``None``. Native bulk work releases the GIL. Matrix construction is a
    one-time O(k^3) symmetric Jacobi decomposition and does not infer a
    covariance matrix from a return stream.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_risk_contributions(self, contributions: Any, *, column: str | None = None) -> "EffectiveNumberOfBets":
        """Append independent non-negative risk contributions."""
        self._state.from_risk_contributions(as_metric_series(contributions, column=column))
        return self

    def from_weights_and_covariance(self, weights: Any, covariance: Any) -> "EffectiveNumberOfBets":
        """Ingest portfolio weights and an aligned covariance matrix."""
        selected_weights = as_metric_series(weights)
        selected_covariance = np.ascontiguousarray(covariance, dtype=np.float64)
        if selected_covariance.ndim != 2:
            raise ValueError("covariance must be a two-dimensional matrix")
        self._state.from_weights_and_covariance(selected_weights, selected_covariance)
        return self

    def append(self, contribution: float) -> "EffectiveNumberOfBets":
        """Append one independent risk contribution and return this metric."""
        self._state.append(float(contribution))
        return self

    def extend(
        self, contributions: Any, *, column: str | None = None
    ) -> "EffectiveNumberOfBets":
        """Append independent risk contributions and return this metric."""
        self._state.extend(as_metric_series(contributions, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the effective bet count, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying contributions."""
        return self._state.compute()

    def reset(self) -> "EffectiveNumberOfBets":
        """Clear contributions, preserve policy, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return valid contribution count delegated to Rust."""
        return len(self._state)


__all__ = ["EffectiveNumberOfBets"]
