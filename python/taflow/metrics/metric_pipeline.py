"""Named lifecycle pipeline for configured metric instances."""
from __future__ import annotations
from typing import Any, Protocol, runtime_checkable
from ._input import as_metric_series

@runtime_checkable
class _Metric(Protocol):

    def append(self, value: float) -> Any:
        ...

    def extend(self, values: Any) -> Any:
        ...

    def compute(self) -> float | int | None:
        ...

    @property
    def value(self) -> float | int | None:
        ...

    def reset(self) -> Any:
        ...

    def __len__(self) -> int:
        ...

class MetricPipeline:
    """Update configured metric instances and return results by custom name.

    Create an empty pipeline, add metrics with ``add``, then select and ingest
    a semantic input domain with an instance ``from_*`` method. Subsequent
    ``append`` and ``extend`` calls use that selected domain. Each metric keeps
    its own configuration and native Rust state; Python only coordinates the
    lifecycle and named results.
    """

    def __init__(self) -> None:
        """Create an empty metric pipeline."""
        self._metrics: dict[str, _Metric] = {}
        self._input_method: str | None = None

    def add(self, name: str, metric: _Metric) -> 'MetricPipeline':
        """Add one configured metric under a unique non-empty result name."""
        if self._input_method is not None:
            raise ValueError('metrics must be added before pipeline input is selected')
        key = str(name)
        if not key:
            raise ValueError('metric name must not be empty')
        if key in self._metrics:
            raise ValueError(f'metric name already exists: {key}')
        if not isinstance(metric, _Metric):
            raise TypeError('metric must implement the metric lifecycle')
        self._metrics[key] = metric
        return self

    @property
    def metrics(self) -> tuple[str, ...]:
        """Return caller-provided metric names in insertion order."""
        return tuple(self._metrics)

    def from_returns(self, returns: Any, *, column: str | None=None) -> 'MetricPipeline':
        """Select decimal simple returns and append them to every metric."""
        return self._from('from_returns', returns, column=column)

    def from_log_returns(self, log_returns: Any, *, column: str | None=None) -> 'MetricPipeline':
        """Select log returns and append them to every metric."""
        return self._from('from_log_returns', log_returns, column=column)

    def from_equity(self, equity: Any, *, column: str | None=None) -> 'MetricPipeline':
        """Select positive equity levels and append them to every metric."""
        return self._from('from_equity', equity, column=column)

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None=None) -> 'MetricPipeline':
        """Select period P&L and append it to every compatible metric."""
        values = as_metric_series(pnl, column=column)
        self._require_input_method('from_pnl', 'period P&L')
        self._select('from_pnl')
        for metric in self._metrics.values():
            method = getattr(metric, 'from_pnl')
            method(values, float(initial_capital))
        return self

    def _from(self, method_name: str, values: Any, *, column: str | None) -> 'MetricPipeline':
        converted = as_metric_series(values, column=column)
        domain = method_name.removeprefix('from_').replace('_', ' ')
        self._require_input_method(method_name, domain)
        self._select(method_name)
        for metric in self._metrics.values():
            method = getattr(metric, method_name)
            method(converted)
        return self

    def _require_input_method(self, method_name: str, domain: str) -> None:
        for metric in self._metrics.values():
            if not callable(getattr(metric, method_name, None)):
                raise TypeError(f'{type(metric).__name__} does not accept {domain} input')

    def _select(self, method_name: str) -> None:
        if self._input_method is None:
            self._input_method = method_name
        elif self._input_method != method_name:
            raise ValueError('pipeline input domain is already selected')

    def append(self, value: float) -> 'MetricPipeline':
        """Append one observation in the selected domain to every metric."""
        if self._input_method is None:
            raise ValueError('call a semantic from_* method before append or extend')
        for metric in self._metrics.values():
            metric.append(float(value))
        return self

    def extend(self, values: Any, *, column: str | None=None) -> 'MetricPipeline':
        """Append observations in the selected domain to every metric."""
        if self._input_method is None:
            raise ValueError('call a semantic from_* method before append or extend')
        converted = as_metric_series(values, column=column)
        for metric in self._metrics.values():
            metric.extend(converted)
        return self

    @property
    def value(self) -> dict[str, float | int | None]:
        """Return current metric values under caller-provided names."""
        return {name: metric.value for name, metric in self._metrics.items()}

    def compute(self) -> dict[str, float | int | None]:
        """Compute current metric values under caller-provided names."""
        return {name: metric.compute() for name, metric in self._metrics.items()}

    def reset(self) -> 'MetricPipeline':
        """Reset every metric while preserving names, configuration, and domain."""
        for metric in self._metrics.values():
            metric.reset()
        return self

    def __len__(self) -> int:
        """Return the common native metric length, or zero when empty."""
        lengths = {len(metric) for metric in self._metrics.values()}
        if not lengths:
            return 0
        if len(lengths) != 1:
            raise RuntimeError('pipeline metrics have inconsistent lengths')
        return lengths.pop()
__all__ = ['MetricPipeline']
