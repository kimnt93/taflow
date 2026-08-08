"""Correctness tests for the causal pipeline graph.

The load-bearing property is that every stateful node is stepped **exactly
once per bar**. A node stepped twice would advance its indicator state twice
and silently corrupt every later value, so these tests count steps with a spy
rather than only comparing outputs.
"""

import numpy as np
import pytest

from taflow import ExponentialMovingAverage, SimpleMovingAverage, AverageTrueRange
from taflow.op import TAPipeline


class SpyState:
    """Wraps an indicator and counts how many times it is stepped."""

    def __init__(self, inner):
        self.inner = inner
        self.steps = 0

    def append(self, *args):
        self.steps += 1
        self.inner.append(*args)
        return self

    @property
    def value(self):
        return self.inner.value

    def reset(self):
        self.steps = 0
        self.inner.reset()


@pytest.fixture
def bars():
    rng = np.random.default_rng(20260808)
    close = np.cumsum(rng.normal(0, 1, 500)) + 100.0
    high = close + rng.random(500)
    low = close - rng.random(500)
    return high, low, close


def test_shared_node_steps_once_per_bar(bars):
    """One indicator feeding several outputs is stepped once, not once per output."""
    _, _, close = bars
    pipe = TAPipeline()
    spy = SpyState(ExponentialMovingAverage(timeperiod=10))
    node = pipe.indicator("ema", spy, pipe.source("close"))

    # Four distinct consumers of the same node.
    pipe.output("direct", node)
    pipe.output("plus", pipe.expression("plus", node + 1.0))
    pipe.output("scaled", pipe.expression("scaled", node * 2.0))
    pipe.output("nested", pipe.expression("nested", (node + node) / (node + 1.0)))

    pipe.extend({"close": close})
    assert spy.steps == len(close)


def test_chained_indicators_step_once_each(bars):
    """An indicator consuming another indicator advances both exactly once."""
    _, _, close = bars
    pipe = TAPipeline()
    first = SpyState(ExponentialMovingAverage(timeperiod=5))
    second = SpyState(SimpleMovingAverage(timeperiod=7))

    a = pipe.indicator("ema", first, pipe.source("close"))
    b = pipe.indicator("sma_of_ema", second, a)
    pipe.output("a", a)
    pipe.output("b", b)
    pipe.output("spread", pipe.expression("spread", a - b))

    pipe.extend({"close": close})
    assert first.steps == len(close)
    assert second.steps == len(close)


def test_multi_input_indicator_steps_once(bars):
    """A three-input indicator sharing sources with others steps once per bar."""
    high, low, close = bars
    pipe = TAPipeline()
    atr = SpyState(AverageTrueRange(timeperiod=14))
    ema = SpyState(ExponentialMovingAverage(timeperiod=10))

    h, l, c = pipe.source("high"), pipe.source("low"), pipe.source("close")
    atr_node = pipe.indicator("atr", atr, h, l, c)
    ema_node = pipe.indicator("ema", ema, c)

    pipe.output("atr", atr_node)
    pipe.output("ratio", pipe.expression("ratio", ema_node / atr_node))
    pipe.output("band", pipe.expression("band", ema_node + 2.0 * atr_node))

    pipe.extend({"high": high, "low": low, "close": close})
    assert atr.steps == len(close)
    assert ema.steps == len(close)


def test_append_steps_once_per_call(bars):
    """Bar-at-a-time dispatch also steps a shared node exactly once."""
    _, _, close = bars
    pipe = TAPipeline()
    spy = SpyState(SimpleMovingAverage(timeperiod=4))
    node = pipe.indicator("sma", spy, pipe.source("close"))
    pipe.output("a", node)
    pipe.output("b", pipe.expression("b", node * 3.0))

    for i, value in enumerate(close[:50], start=1):
        pipe.append({"close": float(value)})
        assert spy.steps == i


def test_pipeline_matches_standalone_indicator(bars):
    """Graph output equals driving the same indicator directly."""
    _, _, close = bars
    pipe = TAPipeline()
    node = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=12),
                          pipe.source("close"))
    pipe.output("ema", node)
    from_pipeline = pipe.extend({"close": close})["ema"]

    standalone = ExponentialMovingAverage(timeperiod=12)
    standalone.extend(close)
    expected = standalone.compute()

    np.testing.assert_array_equal(from_pipeline, expected)


def test_extend_equals_repeated_append(bars):
    """Column dispatch and bar dispatch agree bitwise."""
    high, low, close = bars

    def build():
        pipe = TAPipeline()
        h, l, c = pipe.source("high"), pipe.source("low"), pipe.source("close")
        atr = pipe.indicator("atr", AverageTrueRange(timeperiod=14), h, l, c)
        ema = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=20), c)
        pipe.output("atr", atr)
        pipe.output("z", pipe.expression("z", (c - ema) / atr))
        return pipe

    bulk = build().extend({"high": high, "low": low, "close": close})

    stepwise = build()
    rows = {"atr": [], "z": []}
    for h, l, c in zip(high, low, close):
        out = stepwise.append({"high": float(h), "low": float(l), "close": float(c)})
        rows["atr"].append(out["atr"])
        rows["z"].append(out["z"])

    for key in ("atr", "z"):
        np.testing.assert_array_equal(bulk[key], np.array(rows[key]))


def test_chunked_extend_matches_single_extend(bars):
    """Splitting the input across extend calls does not change results."""
    _, _, close = bars

    def build():
        pipe = TAPipeline()
        node = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=15),
                              pipe.source("close"))
        pipe.output("ema", node)
        return pipe

    whole = build().extend({"close": close})["ema"]

    chunked, pipe = [], build()
    for start in range(0, len(close), 37):
        chunked.append(pipe.extend({"close": close[start:start + 37]})["ema"])

    np.testing.assert_array_equal(whole, np.concatenate(chunked))


def test_reset_restores_initial_state(bars):
    """reset() clears stateful nodes so a replay reproduces the first run."""
    _, _, close = bars
    pipe = TAPipeline()
    node = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=9),
                          pipe.source("close"))
    pipe.output("ema", node)

    first = pipe.extend({"close": close})["ema"]
    pipe.reset()
    second = pipe.extend({"close": close})["ema"]

    np.testing.assert_array_equal(first, second)


def test_warmup_is_nan_and_length_is_preserved(bars):
    """Outputs stay aligned with inputs, warm-up reported as NaN."""
    _, _, close = bars
    pipe = TAPipeline()
    node = pipe.indicator("sma", SimpleMovingAverage(timeperiod=30),
                          pipe.source("close"))
    pipe.output("sma", node)

    result = pipe.extend({"close": close})["sma"]
    assert len(result) == len(close)
    assert np.all(np.isnan(result[:29]))
    assert not np.isnan(result[29])


def test_outputs_property_lists_registered_names(bars):
    _, _, close = bars
    pipe = TAPipeline()
    node = pipe.indicator("sma", SimpleMovingAverage(timeperiod=5),
                          pipe.source("close"))
    pipe.output("a", node)
    pipe.output("b", pipe.expression("b", node + 1.0))
    assert set(pipe.outputs) == {"a", "b"}


def test_mismatched_input_lengths_raise():
    pipe = TAPipeline()
    node = pipe.indicator("sma", SimpleMovingAverage(timeperiod=3),
                          pipe.source("close"))
    pipe.output("sma", node)
    with pytest.raises(ValueError):
        pipe.extend({"close": [1.0, 2.0, 3.0], "other": [1.0, 2.0]})


def test_unreachable_node_is_not_stepped(bars):
    """A node no output depends on is never advanced.

    This documents real behaviour that is easy to trip over: the graph is
    demand-driven from the registered outputs, so an indicator built but not
    wired to an output stays at bar zero. Register it as an output (or make
    something depend on it) if its state must track the feed.
    """
    _, _, close = bars
    pipe = TAPipeline()
    used = SpyState(SimpleMovingAverage(timeperiod=5))
    unused = SpyState(SimpleMovingAverage(timeperiod=5))

    pipe.output("used", pipe.indicator("used", used, pipe.source("close")))
    pipe.indicator("unused", unused, pipe.source("close"))

    pipe.extend({"close": close})
    assert used.steps == len(close)
    assert unused.steps == 0


def test_chained_indicator_propagates_warmup_nan(bars):
    """Documents a real defect: upstream warm-up NaN poisons a summing state.

    A downstream running-sum indicator does ``sum += new - old``; once ``sum``
    is NaN it stays NaN forever, so chaining onto SMA/VAR/STDDEV yields an
    all-NaN series rather than a warm-up prefix. Chain only onto states that
    tolerate NaN, or strip the upstream warm-up first. Change this test when
    the behaviour is fixed.
    """
    _, _, close = bars
    pipe = TAPipeline()
    ema = pipe.indicator("ema", ExponentialMovingAverage(timeperiod=5),
                         pipe.source("close"))
    chained = pipe.indicator("chained", SimpleMovingAverage(timeperiod=7), ema)
    pipe.output("chained", chained)

    result = pipe.extend({"close": close})["chained"]
    assert np.all(np.isnan(result)), "expected the documented all-NaN poisoning"

    # The workaround: feed only the warmed upstream values.
    upstream = ExponentialMovingAverage(timeperiod=5)
    upstream.extend(close)
    warm = upstream.compute()
    warm = warm[~np.isnan(warm)]
    downstream = SimpleMovingAverage(timeperiod=7)
    downstream.extend(warm)
    assert not np.isnan(downstream.compute()[-1])
