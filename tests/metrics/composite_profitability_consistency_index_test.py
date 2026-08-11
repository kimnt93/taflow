import numpy as np
import pandas as pd
import pytest
import quantstats.stats as qs
from taflow.metrics import CompositeProfitabilityConsistencyIndex


@pytest.mark.parametrize("values",[np.array([.1,-.05,.2,-.1]),np.linspace(-.05,.06,101),np.array([np.nan,.02,-.01,0,.03])])
def test_matches_quantstats(values):
    usable=values[~np.isnan(values)];series=pd.Series(usable);expected=float(qs.cpc_index(series,prepare_returns=False));actual=CompositeProfitabilityConsistencyIndex.from_returns(values).compute();assert actual==pytest.approx(expected,rel=1e-12,abs=1e-14)


def test_trades_lifecycle_edges():
    values=np.array([10.,-5.,20.,-10.,0.]);expected=CompositeProfitabilityConsistencyIndex.from_trades(values).compute();metric=CompositeProfitabilityConsistencyIndex.from_trades([]).append(10).extend(values[1:]);assert metric.compute()==pytest.approx(expected);assert metric.reset().extend(values).compute()==pytest.approx(expected)
    assert CompositeProfitabilityConsistencyIndex.from_returns([.1,0]).compute() is None
    with pytest.raises(TypeError):CompositeProfitabilityConsistencyIndex()
