use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::AnnualizedReturn as State, MetricInputKind, NanPolicy};

/// Native state backing `taflow.metrics.AnnualizedReturn`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct AnnualizedReturn {
    inner: State,
}

#[pymethods]
impl AnnualizedReturn {
    #[new]
    #[pyo3(signature = (input_kind, periods_per_year=252.0, initial_equity=None, nan_policy="omit"))]
    fn new(
        input_kind: &str,
        periods_per_year: f64,
        initial_equity: Option<f64>,
        nan_policy: &str,
    ) -> PyResult<Self> {
        let kind = match input_kind {
            "returns" => MetricInputKind::Returns,
            "log_returns" => MetricInputKind::LogReturns,
            "equity" => MetricInputKind::Equity,
            "pnl" => MetricInputKind::PeriodPnl {
                initial_equity: initial_equity.ok_or_else(|| {
                    PyValueError::new_err("initial_equity is required for period P&L")
                })?,
            },
            _ => {
                return Err(PyValueError::new_err(
                    "input_kind must be 'returns', 'log_returns', 'equity', or 'pnl'",
                ));
            }
        };
        let policy = NanPolicy::try_from(nan_policy)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        let inner = State::new(kind, periods_per_year, policy)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self { inner })
    }

    fn append(&mut self, value: f64) -> PyResult<Option<f64>> {
        self.inner
            .append(value)
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        values: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<Option<f64>> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(values))
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self) -> Option<f64> {
        self.inner.compute()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
