use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::GrossLoss as State, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}
/// Native state backing `taflow.metrics.GrossLoss`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct GrossLoss {
    inner: State,
}

#[pymethods]
impl GrossLoss {
    #[new]
    #[pyo3(signature = (nan_policy="omit"))]
    fn new(nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(NanPolicy::try_from(nan_policy).map_err(value_error)?)
                .map_err(value_error)?,
        })
    }

    fn from_pnl(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_pnl(values))
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_trades(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_trades(values))
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
