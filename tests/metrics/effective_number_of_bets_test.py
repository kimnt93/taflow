from __future__ import annotations

import numpy as np
import pytest

from taflow.metrics.effective_number_of_bets import EffectiveNumberOfBets


def _numpy_pca_effective_bets(weights: np.ndarray, covariance: np.ndarray) -> float:
    eigenvalues, eigenvectors = np.linalg.eigh(covariance)
    exposures = eigenvectors.T @ weights
    contributions = np.maximum(eigenvalues, 0.0) * exposures**2
    probabilities = contributions / contributions.sum()
    positive = probabilities > 0.0
    return float(np.exp(-np.sum(probabilities[positive] * np.log(probabilities[positive]))))


@pytest.mark.parametrize(
    ("weights", "covariance"),
    [
        (np.array([0.5, 0.5]), np.eye(2)),
        (np.array([0.6, 0.4]), np.array([[0.04, 0.012], [0.012, 0.09]])),
        (np.array([1.0, 0.0, 0.0]), np.diag([0.01, 0.04, 0.09])),
        (np.array([0.2, 0.3, 0.5]), np.array([[0.04, 0.01, -0.002], [0.01, 0.06, 0.008], [-0.002, 0.008, 0.09]])),
    ],
)
def test_weights_and_covariance_match_numpy_eigh(
    weights: np.ndarray, covariance: np.ndarray
) -> None:
    expected = _numpy_pca_effective_bets(weights, covariance)
    actual = EffectiveNumberOfBets.from_weights_and_covariance(
        weights, covariance
    ).compute()
    assert actual == pytest.approx(expected, rel=2e-10, abs=2e-12)


def test_contribution_stream_matches_entropy_oracle_and_lifecycle() -> None:
    contributions = np.array([0.2, 0.3, 0.0, 0.5])
    probabilities = contributions / contributions.sum()
    expected = float(
        np.exp(-np.sum(probabilities[probabilities > 0] * np.log(probabilities[probabilities > 0])))
    )
    state = EffectiveNumberOfBets.from_risk_contributions([])
    assert state.value is None
    assert state.append(contributions[0]) is state
    assert state.extend(contributions[1:]) is state
    assert state.compute() == pytest.approx(expected)
    assert state.compute() == state.compute()
    assert len(state) == len(contributions)
    assert state.reset() is state and len(state) == 0
    assert state.extend(contributions).compute() == pytest.approx(expected)


def test_validation_and_semantic_factory() -> None:
    assert EffectiveNumberOfBets.from_risk_contributions([]).compute() is None
    assert EffectiveNumberOfBets.from_risk_contributions([0.0]).compute() is None
    with pytest.raises(ValueError):
        EffectiveNumberOfBets.from_risk_contributions([-0.1])
    with pytest.raises(ValueError):
        EffectiveNumberOfBets.from_weights_and_covariance([0.5, 0.5], np.eye(3))
    with pytest.raises(ValueError):
        EffectiveNumberOfBets.from_weights_and_covariance(
            [0.5, 0.5], [[1.0, 0.2], [0.0, 1.0]]
        )
    with pytest.raises(TypeError):
        EffectiveNumberOfBets()
