use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::NetProfit as State, NanPolicy};
fn err(e: impl ToString) -> PyErr {
    PyValueError::new_err(e.to_string())
}
/// Native state backing `taflow.metrics.NetProfit`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct NetProfit {
    inner: State,
}
#[pymethods]
impl NetProfit {
    #[new]
    #[pyo3(signature = (nan_policy="omit"))]
    fn new(nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(NanPolicy::try_from(nan_policy).map_err(err)?).map_err(err)?,
        })
    }

    fn from_pnl(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_pnl(values))
            .map(|_| ())
            .map_err(err)
    }

    fn from_trades(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_trades(values))
            .map(|_| ())
            .map_err(err)
    }

    fn append(&mut self, value: f64) -> PyResult<Option<f64>> {
        self.inner.append(value).map_err(err)
    }
    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let v = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(v))
            .map(|_| ())
            .map_err(err)
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn compute(&self) -> Option<f64> {
        self.inner.compute()
    }
    fn reset(&mut self) {
        self.inner.reset()
    }
    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
