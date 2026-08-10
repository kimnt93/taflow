use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::{DemandIndex as State, DemandIndexValue};
#[pyclass]
pub struct DemandIndex {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl DemandIndex {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64, close: f64, volume: f64) -> Option<f64> {
        let v = self.inner.append(high, low, close, volume);
        self.output
            .push(v.map(|x: DemandIndexValue| x.demand).unwrap_or(f64::NAN));
        v.map(|x| x.demand)
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low, close, volume) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
        );
        if high.len() != low.len() || high.len() != close.len() || high.len() != volume.len() {
            return Err(PyValueError::new_err(
                "OHLCV inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (((h, l), c), v) in high.iter().zip(low).zip(close).zip(volume) {
                self.append(*h, *l, *c, *v);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value().map(|x| x.demand)
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
