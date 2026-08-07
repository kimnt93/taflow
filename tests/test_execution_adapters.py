import numpy as np
import pytest

import taflow
from taflow.talib.state import EMA


def test_pipeline_one_pass_and_common_subexpression():
    pipeline = taflow.Pipeline()
    close = pipeline.source("close")
    ema = pipeline.indicator("ema", EMA(3), close)
    shared = ema + 1.0
    pipeline.output("plus", shared)
    pipeline.output("double", shared * 2.0)

    values = np.arange(1.0, 8.0)
    result = pipeline.extend({"close": values})
    expected = EMA(3).extend(values)
    np.testing.assert_allclose(result["plus"], expected + 1.0, equal_nan=True)
    np.testing.assert_allclose(result["double"], (expected + 1.0) * 2.0, equal_nan=True)


def test_shared_indicator_is_dispatched_once_per_bar():
    class Counter:
        def __init__(self): self.calls = 0
        def append(self, value): self.calls += 1; return value
        def reset(self): self.calls = 0

    counter = Counter()
    pipeline = taflow.Pipeline()
    node = pipeline.indicator("counter", counter, pipeline.source("close"))
    pipeline.output("a", node + 1)
    pipeline.output("b", node * 2)
    pipeline.extend({"close": [1.0, 2.0, 3.0]})
    assert counter.calls == 3


def test_pipeline_chunk_invariance_and_alignment():
    values = np.arange(1.0, 40.0)

    def make():
        p = taflow.Pipeline()
        p.output("ema", p.indicator("ema", EMA(5), p.source("close")))
        return p

    full = make().extend({"close": values})["ema"]
    chunked = make()
    pieces = [chunked.extend({"close": part})["ema"] for part in np.array_split(values, 4)]
    np.testing.assert_array_equal(np.concatenate(pieces), full)
    with pytest.raises(ValueError): make().extend({"close": values, "x": values[:-1]})


def test_list_and_numpy_adapters():
    values = [1, 2, 3]
    array = taflow.adapt_input(values, adapter="list")
    assert array.dtype == np.float64
    assert taflow.PythonListAdapter.output(array) == values
    np.testing.assert_array_equal(taflow.NumpyAdapter.input(values), array)
    assert taflow.AdapterGateway.output(array, adapter="list") == values


def test_optional_adapters_fail_lazily_or_convert():
    for adapter in (taflow.ArrowAdapter, taflow.PolarsAdapter):
        try:
            converted = adapter.input([1, 2, 3])
        except ImportError:
            continue
        assert len(converted) == 3
