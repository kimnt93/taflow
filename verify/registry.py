"""Shared registry: map TA-Lib names to canonical taflow classes.

taflow no longer ships a TA-Lib compatibility module, so verification and
benchmarking drive the canonical CamelCase classes and translate:

  - names   — parsed from the master table in /CHECK.md (the maintained
              contract), resolved against the live module with synonym
              expansion (e.g. ``rolling_std`` -> ``RollingStandardDeviation``);
  - inputs  — TA-Lib input roles (from ``talib.abstract``) matched to the
              class ``extend`` signature positionally;
  - params  — TA-Lib parameter names mapped to constructor keywords via
              exact, synonym, or underscore-normalized matching.

Rows that cannot be resolved are reported (class missing / param mismatch),
never silently skipped — surfacing drift between CHECK.md and the code is a
feature.
"""

from __future__ import annotations

import inspect
import re
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

CHECK_MD = Path(__file__).resolve().parent.parent / "CHECK.md"

# Whole-name replacements for table rows whose class names expand
# differently than token-by-token camel-casing.
WHOLE_NAME = {
    "acos": "math_acos", "asin": "math_asin", "atan": "math_atan",
    "ceil": "math_ceil", "cos": "math_cos", "cosh": "math_cosh",
    "exp": "math_exp", "floor": "math_floor", "ln": "math_ln",
    "log10": "math_log10", "sin": "math_sin", "sinh": "math_sinh",
    "sqrt": "math_sqrt", "tan": "math_tan", "tanh": "math_tanh",
    "add": "math_add", "sub": "math_subtract",
    "mult": "math_multiply", "div": "math_divide",
    "avg_price": "average_price",
    "kvo": "klinger_volume_oscillator",
    "mcginley": "mcginley_dynamic",
    "vidya": "variable_index_dynamic_average",
    "laguerre_rsi": "laguerre_relative_strength_index",
    "rmi": "relative_momentum_index",
    "jma": "jurik_moving_average",
    "td_sequential": "tom_de_mark_sequential",
    "fib_retracement": "fibonacci_retracement",
    "anchored_vwap": "anchored_volume_weighted_average_price",
}

# Token-level synonym expansion applied inside snake names.
TOKEN_SYNONYMS = {
    "std": "standard_deviation",
    "var": "variance",
    "corr": "correlation",
    "cov": "covariance",
    "avgdev": "average_deviation",
    "linreg": "linear_regression",
    "tsf": "time_series_forecast",
    "ewm": "exponentially_weighted",
    "vwap": "volume_weighted_average_price",
}

# TA-Lib parameter name -> candidate constructor keyword names (tried in
# order, before falling back to underscore-normalized equality).
PARAM_SYNONYMS = {
    "timeperiod": ("timeperiod", "period", "time_period"),
    "nbdevup": ("deviations_up", "nbdevup"),
    "nbdevdn": ("deviations_down", "nbdevdn"),
    "matype": ("moving_average_type", "matype", "average_type"),
    "fastmatype": ("fast_average_type", "fast_moving_average_type"),
    "slowmatype": ("slow_average_type", "slow_moving_average_type"),
    "signalmatype": ("signal_average_type", "signal_moving_average_type"),
    "fastd_matype": ("fast_d_average_type",),
    "slowk_matype": ("slow_k_average_type",),
    "slowd_matype": ("slow_d_average_type",),
    "vfactor": ("volume_factor", "v_factor"),
    "startvalue": ("start_value",),
    "offsetonreverse": ("offset_on_reverse",),
    "accelerationinitlong": ("acceleration_init_long",),
    "accelerationlong": ("acceleration_long",),
    "accelerationmaxlong": ("acceleration_max_long",),
    "accelerationinitshort": ("acceleration_init_short",),
    "accelerationshort": ("acceleration_short",),
    "accelerationmaxshort": ("acceleration_max_short",),
    "minperiod": ("min_period",),
    "maxperiod": ("max_period",),
    "fastlimit": ("fast_limit",),
    "slowlimit": ("slow_limit",),
}

# Input domains narrower than a price series.
INPUT_DOMAIN_OVERRIDES = {"ACOS": "unit", "ASIN": "unit"}

# Series-typed constructor/extend parameter names (never mapped as params).
SERIES_PARAM_NAMES = {
    "_input", "input", "values", "close", "high", "low", "open", "_open",
    "volume", "left", "right", "periods", "real", "price", "column",
}


def _norm(name: str) -> str:
    return name.replace("_", "").lower()


def parse_master_table() -> list[tuple[str, str]]:
    """Return (snake_name, talib_name_or_'_') rows from CHECK.md §2.5."""
    text = CHECK_MD.read_text()
    return [
        (m.group(1), m.group(2))
        for m in re.finditer(
            r"^\| \[(?:x| )\] \| ([a-z0-9_]+) \| [a-z0-9_]+ "
            r"\| ([A-Z0-9_]+|_) \|", text, re.M)
    ]


def resolve_class(snake: str):
    """Resolve a snake-case table name to a live taflow class (or None)."""
    import taflow

    by_norm = {c.lower(): c for c in dir(taflow) if c[:1].isupper()}
    candidates = [snake]
    if snake in WHOLE_NAME:
        candidates.append(WHOLE_NAME[snake])
    candidates.append(
        "_".join(TOKEN_SYNONYMS.get(tok, tok) for tok in snake.split("_")))
    for cand in candidates:
        hit = by_norm.get(_norm(cand))
        if hit:
            return getattr(taflow, hit)
    return None


@dataclass
class Spec:
    """One function: taflow class + TA-Lib translation."""

    snake: str
    talib_name: str | None            # None => taflow-only
    cls: type | None
    ctor_kwargs: dict = field(default_factory=dict)
    series_args: tuple[str, ...] = ()  # extend arg names, in call order
    input_roles: tuple[str, ...] = ()  # TA-Lib roles, same order
    domain: str = "prices"
    error: str | None = None
    warnings: list = field(default_factory=list)
    lookback: int = 0

    # -- construction ------------------------------------------------------

    @classmethod
    def build(cls, snake: str, talib_name: str | None) -> "Spec":
        spec = cls(snake=snake, talib_name=talib_name,
                   cls=resolve_class(snake))
        if spec.cls is None:
            spec.error = "no matching taflow class"
            return spec
        try:
            sig = inspect.signature(spec.cls.__init__)
        except (TypeError, ValueError):
            spec.error = "constructor signature unavailable"
            return spec
        ctor_params = {p for p in sig.parameters if p != "self"}

        try:
            ext_sig = inspect.signature(spec.cls.extend)
            # Ignore optional dataframe-column selector kwargs.
            spec.series_args = tuple(
                p for p in ext_sig.parameters
                if p != "self" and not p.endswith("column"))
        except (AttributeError, TypeError, ValueError):
            spec.error = "no extend method"
            return spec

        if talib_name:
            spec._bind_talib(talib_name, ctor_params)
        else:
            spec.input_roles = tuple(
                "close" if a in ("_input", "input", "values") else a
                for a in spec.series_args)
        return spec

    def _bind_talib(self, name: str, ctor_params: set[str]) -> None:
        from talib import abstract

        info = abstract.Function(name).info
        self.lookback = int(abstract.Function(name).lookback)
        self.domain = INPUT_DOMAIN_OVERRIDES.get(name, "prices")

        roles: list[str] = []
        for role, value in info["input_names"].items():
            if isinstance(value, (list, tuple)):
                roles.extend(value)
            else:
                roles.append(role if role.startswith("price") else value)
        if len(roles) != len(self.series_args):
            self.error = (f"input arity mismatch: talib {roles} vs "
                          f"extend{self.series_args}")
            return
        self.input_roles = tuple(roles)

        norm_ctor = {_norm(p): p for p in ctor_params
                     if p not in SERIES_PARAM_NAMES}
        for talib_param, value in info["parameters"].items():
            target = None
            for cand in PARAM_SYNONYMS.get(talib_param, ()):
                if cand in ctor_params:
                    target = cand
                    break
            if target is None:
                target = norm_ctor.get(_norm(talib_param))
            if target is None:
                # Parameter absent from the canonical class (e.g. TA-Lib's
                # `penetration` / `nbdev`). At TA-Lib defaults the comparison
                # is still valid; record a warning instead of failing.
                self.warnings.append(
                    f"talib param {talib_param!r} (default {value!r}) has no "
                    "constructor counterpart — compared at defaults only")
                continue
            self.ctor_kwargs[target] = value

    # -- data --------------------------------------------------------------

    def arrays(self, data: dict, n: int) -> list[np.ndarray]:
        out = []
        for role in self.input_roles:
            if role in ("price", "real", "close"):
                key = "unit" if self.domain == "unit" else "close"
            elif role == "price0":
                key = "close"
            elif role == "price1":
                key = "close2"
            elif role in data:
                key = role
            else:
                key = "close"
            out.append(np.ascontiguousarray(data[key][:n]))
        return out

    # -- state adapters (handle fluent and value-returning APIs) -----------

    def new_state(self):
        return self.cls(**self.ctor_kwargs)

    @staticmethod
    def extend(state, arrays) -> tuple[np.ndarray, ...]:
        result = state.extend(*arrays)
        if result is state:
            result = state.compute()
        return result if isinstance(result, tuple) else (result,)

    @staticmethod
    def append_value(state, bar):
        result = state.append(*bar)
        if result is state:
            result = getattr(state, "value", None)
        return result


def build_registry() -> dict[str, Spec]:
    """All rows from the master table keyed by TA-Lib name or snake name."""
    specs: dict[str, Spec] = {}
    for snake, talib_name in parse_master_table():
        key = talib_name if talib_name != "_" else snake
        specs[key] = Spec.build(snake,
                                talib_name if talib_name != "_" else None)
    return specs


# ---------------------------------------------------------------------------
# Shared deterministic data generator (mean-reverting log-price OHLCV)
# ---------------------------------------------------------------------------

def make_data(n: int, seed: int = 42) -> dict[str, np.ndarray]:
    def ar1(offset: int) -> np.ndarray:
        rng = np.random.default_rng(seed + offset)
        noise = rng.normal(0.0, 0.02, n)
        decay = 1.0 - 0.001
        block = 4096
        pows = decay ** np.arange(block)
        inv_pows = decay ** -np.arange(block)
        x = np.empty(n)
        carry = 0.0
        for start in range(0, n, block):
            chunk = noise[start:start + block]
            m = len(chunk)
            conv = pows[:m] * np.cumsum(chunk * inv_pows[:m])
            x[start:start + m] = conv + carry * decay * pows[:m]
            carry = x[start + m - 1]
        return 100.0 * np.exp(x)

    close = ar1(0)
    rng = np.random.default_rng(seed + 1000)
    spread = close * 0.01
    high = close + rng.uniform(0.0, 1.0, n) * spread
    low = close - rng.uniform(0.0, 1.0, n) * spread
    open_ = low + rng.uniform(0.0, 1.0, n) * (high - low)
    unit_noise = np.random.default_rng(seed + 2000).normal(0.0, 0.05, n)
    return {
        "open": open_, "high": high, "low": low, "close": close,
        "volume": rng.uniform(1e5, 1e6, n),
        "close2": ar1(3000),
        "periods": np.random.default_rng(seed + 4000).uniform(2.0, 30.0, n),
        "unit": np.clip(np.cumsum(unit_noise) % 1.8 - 0.9, -0.99, 0.99),
    }
