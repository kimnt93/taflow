use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::ProbabilisticSharpeRatio as State, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}
/// Native state backing `taflow.metrics.ProbabilisticSharpeRatio`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct ProbabilisticSharpeRatio {
    inner: State,
}

#[pymethods]
impl ProbabilisticSharpeRatio {
    #[new]
    #[pyo3(signature = (periods_per_year=252.0, annual_risk_free_rate=0.0, annual_benchmark_sharpe_ratio=0.0, nan_policy="omit"))]
    fn new(
        periods_per_year: f64,
        annual_risk_free_rate: f64,
        annual_benchmark_sharpe_ratio: f64,
        nan_policy: &str,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                periods_per_year,
                annual_risk_free_rate,
                annual_benchmark_sharpe_ratio,
                NanPolicy::try_from(nan_policy).map_err(value_error)?,
            )
            .map_err(value_error)?,
        })
    }

    fn from_returns(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_returns(values))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_log_returns(
        &mut self,
        py: Python<'_>,
        values: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_log_returns(values))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_equity(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_equity(values))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_pnl(
        &mut self,
        py: Python<'_>,
        values: PyReadonlyArray1<'_, f64>,
        initial_capital: f64,
    ) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_pnl(values, initial_capital))
            .map(|_| ())
            .map_err(value_error)
    }

    fn append(&mut self, value: f64) -> PyResult<Option<f64>> {
        self.inner.append(value).map_err(value_error)
    }

    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(values))
            .map(|_| ())
            .map_err(value_error)
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
