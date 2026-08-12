use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::OmegaRatio as State, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}
/// Native state backing `taflow.metrics.OmegaRatio`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct OmegaRatio {
    inner: State,
}

#[pymethods]
impl OmegaRatio {
    #[new]
    #[pyo3(signature = (periods_per_year=252.0, annual_required_return=0.0, nan_policy="omit"))]
    fn new(periods_per_year: f64, annual_required_return: f64, nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                periods_per_year,
                annual_required_return,
                NanPolicy::try_from(nan_policy)
                    .map_err(|error| PyValueError::new_err(error.to_string()))?,
            )
            .map_err(|error| PyValueError::new_err(error.to_string()))?,
        })
    }

    fn from_returns(&mut self, py: Python<'_>, returns: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let returns = returns.as_slice()?;
        py.allow_threads(|| self.inner.from_returns(returns))
            .map(|_| ())
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    fn from_log_returns(
        &mut self,
        py: Python<'_>,
        values: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_log_returns(values))
            .map(|_| ())
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    fn from_equity(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_equity(values))
            .map(|_| ())
            .map_err(|error| PyValueError::new_err(error.to_string()))
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
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    fn append(&mut self, value: f64) -> PyResult<Option<f64>> {
        self.inner
            .append(value)
            .map_err(|error| PyValueError::new_err(error.to_string()))
    }

    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(values))
            .map(|_| ())
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
