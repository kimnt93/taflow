#!/usr/bin/env python3
"""Regenerate the shared Python and Rust metric API reference.

Python signatures are introspected from the installed public metric classes.
Rust signatures are read from the canonical metric implementation files.
Metric semantics come from the correctness registry rather than generated
evaluation reports.

    python scripts/gen_metrics_doc.py
"""

import inspect
import pathlib
import re

from verification.metrics.registry import METRICS

ROOT = pathlib.Path(__file__).resolve().parent.parent


def python_signature(callable_object, *, drop_self=False):
    """Return a complete public signature without the return annotation."""

    try:
        signature = inspect.signature(callable_object, eval_str=True)
    except (TypeError, ValueError):
        return "—"
    if drop_self:
        parameters = tuple(signature.parameters.values())
        if parameters and parameters[0].name == "self":
            signature = signature.replace(parameters=parameters[1:])
    return str(signature).rsplit(" -> ", 1)[0]


def rust_method_signature(source, method_name):
    """Return the exact parameter list for one public Rust inherent method."""

    match = re.search(rf"pub\s+fn\s+{re.escape(method_name)}\s*\(", source)
    if match is None:
        return "—"
    start = source.index("(", match.start())
    depth = 0
    end = None
    for index in range(start, len(source)):
        if source[index] == "(":
            depth += 1
        elif source[index] == ")":
            depth -= 1
            if depth == 0:
                end = index
                break
    if end is None:
        raise ValueError(f"unclosed {method_name} signature")
    parameters = re.sub(r"\s+", " ", source[start + 1 : end]).strip()
    parameters = re.sub(r"\s*,\s*", ", ", parameters)
    parameters = parameters.rstrip(", ")
    return f"{method_name}({parameters})"


def rust_signature_from_python(method_name, method):
    """Render macro-generated Rust input parameters from the public contract."""

    signature = inspect.signature(method)
    rendered = ["&mut self"]
    for parameter in signature.parameters.values():
        if parameter.name in {"self", "column"}:
            continue
        annotation = str(parameter.annotation).strip("'\"")
        if annotation in {"float", "<class 'float'>"}:
            rust_type = "f64"
        elif annotation in {"int", "<class 'int'>"}:
            rust_type = "usize"
        elif annotation in {"bool", "<class 'bool'>"}:
            rust_type = "bool"
        else:
            rust_type = "&[f64]"
        rendered.append(f"{parameter.name}: {rust_type}")
    return f"{method_name}({', '.join(rendered)})"


def code(value):
    """Render one Markdown table cell as inline code."""

    escaped = value.replace("|", "\\|")
    return f"`{escaped}`"


def main():
    rows = []
    for spec in sorted(METRICS, key=lambda item: item.class_name):
        cls = spec.load_class()
        rust_path = (
            ROOT
            / "crates"
            / "taflow-metrics"
            / "src"
            / "metrics"
            / f"{spec.module}.rs"
        )
        rust_source = rust_path.read_text()
        python_inputs = "<br>".join(
            code(
                f"{method_name}"
                f"{python_signature(getattr(cls, method_name), drop_self=True)}"
            )
            for method_name in spec.input_methods
        )
        rust_input_signatures = []
        for method_name in spec.input_methods:
            signature = rust_method_signature(rust_source, method_name)
            if signature == "—":
                signature = rust_signature_from_python(
                    method_name, getattr(cls, method_name)
                )
            rust_input_signatures.append(code(signature))
        rust_inputs = "<br>".join(rust_input_signatures)
        rows.append(
            {
                "class": spec.class_name,
                "python_constructor": python_signature(cls),
                "rust_constructor": rust_method_signature(rust_source, "new"),
                "python_inputs": python_inputs,
                "rust_inputs": rust_inputs,
                "python_streaming": "<br>".join(
                    code(
                        f"{method_name}"
                        f"{python_signature(getattr(cls, method_name), drop_self=True)}"
                    )
                    for method_name in ("append", "extend")
                ),
                "rust_streaming": "<br>".join(
                    code(rust_method_signature(rust_source, method_name))
                    for method_name in ("append", "extend")
                ),
                "output": spec.output_type,
                "formula": spec.formula,
                "minimum": spec.minimum_observations,
            }
        )

    out = [
        "# TAFlow metric reference\n",
        f"**{len(rows)}** canonical strategy, risk, trade, and portfolio metric classes.\n",
        "> Generated by `scripts/gen_metrics_doc.py` from the installed package, "
        "canonical Rust sources, and correctness registry. Do not edit by hand.\n",
        "Classes are sorted alphabetically by their full canonical names. Python "
        "signatures include annotations, keyword-only markers, and every default "
        "value. Rust has no default arguments, so its constructor and semantic "
        "input parameters are all explicit.\n",
        "## Python usage\n",
        """```python
from taflow.metrics import SharpeRatio

returns = [0.01, -0.004, 0.006, 0.002]
next_return = 0.003
sharpe_ratio = SharpeRatio(
    periods_per_year=252.0,
    annual_risk_free_rate=0.03,
    nan_policy="omit",
).from_returns(returns)

sharpe_ratio.append(next_return)
current_value = sharpe_ratio.value
computed_value = sharpe_ratio.compute()
sharpe_ratio.reset()
```

Constructors contain configuration only. Select exactly one semantic input
domain with a documented `from_*` method, then continue through `append` or
`extend`. The accepted input method signatures are listed per class below.
""",
        "## Rust usage\n",
        """```rust
use taflow_metrics::metrics::SharpeRatio;
use taflow_metrics::NanPolicy;

fn calculate_sharpe_ratio(
    returns: &[f64],
) -> taflow_metrics::MetricResult<Option<f64>> {
    let mut sharpe_ratio = SharpeRatio::new(252.0, 0.03, NanPolicy::Omit)?;
    sharpe_ratio.from_returns(returns)?;
    Ok(sharpe_ratio.compute())
}
```

Rust constructors require every configuration parameter. Semantic input
methods accept borrowed slices; `append`, `extend`, `value`, `compute`,
`reset`, `len`, and `is_empty` operate on the same persistent state.
""",
        "## Alphabetical class reference\n",
        "| Class | Python constructor | Python semantic inputs | Python streaming | Rust constructor | Rust semantic inputs | Rust streaming | Output | Minimum observations | Definition |",
        "|---|---|---|---|---|---|---|---|---:|---|",
    ]
    for row in rows:
        out.append(
            f"| `{row['class']}` | {code(row['python_constructor'])} | "
            f"{row['python_inputs']} | {row['python_streaming']} | "
            f"{code(row['rust_constructor'])} | {row['rust_inputs']} | "
            f"{row['rust_streaming']} | {code(row['output'])} | "
            f"{row['minimum']} | {row['formula']} |"
        )
    out.append("")

    target = ROOT / "docs" / "METRICS.md"
    target.write_text("\n".join(out))
    print(f"wrote {target.relative_to(ROOT)}: {len(rows)} classes")


if __name__ == "__main__":
    main()
