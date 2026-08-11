use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{
    metrics::CompositeProfitabilityConsistencyIndex as State, MetricInputKind, NanPolicy,
};
fn err(e: impl ToString) -> PyErr {
    PyValueError::new_err(e.to_string())
}
fn kind(n: &str) -> PyResult<MetricInputKind> {
    match n {
        "returns" => Ok(MetricInputKind::Returns),
        "trades" => Ok(MetricInputKind::Trades),
        _ => Err(PyValueError::new_err(
            "unsupported CompositeProfitabilityConsistencyIndex input mode",
        )),
    }
}
/// Native state backing `taflow.metrics.CompositeProfitabilityConsistencyIndex`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct CompositeProfitabilityConsistencyIndex {
    inner: State,
}
#[pymethods]
impl CompositeProfitabilityConsistencyIndex {
    #[new]
    #[pyo3(signature=(input_mode,nan_policy="omit"))]
    fn new(input_mode: &str, nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                kind(input_mode)?,
                NanPolicy::try_from(nan_policy).map_err(err)?,
            )
            .map_err(err)?,
        })
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
