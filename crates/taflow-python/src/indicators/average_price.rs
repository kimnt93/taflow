use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::AveragePrice as State;

#[pyclass]
pub struct AveragePrice {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl AveragePrice {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: State::new().map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        let value = self.inner.append(open, high, low, close);
        self.outputs.push(value);
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (open, high, low, close) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
        );
        if [high.len(), low.len(), close.len()]
            .iter()
            .any(|&len| len != open.len())
        {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (((&o, &h), &l), &c) in open.iter().zip(high).zip(low).zip(close) {
                self.append(o, h, l, c);
            }
        });
        Ok(())
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.outputs.clone())
    }
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
