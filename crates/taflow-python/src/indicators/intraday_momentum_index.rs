use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::IntradayMomentumIndex as State;

#[pyclass]
pub struct IntradayMomentumIndex {
    inner: State,
    output: Vec<f64>,
}

#[pymethods]
impl IntradayMomentumIndex {
    #[new]
    #[pyo3(signature = (timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, open: f64, close: f64) -> Option<f64> {
        let value = self.inner.append(open, close);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (open, close) = (open.as_slice()?, close.as_slice()?);
        let (inner, output) = (&mut self.inner, &mut self.output);
        py.allow_threads(|| {
            inner
                .extend_slice_into(open, close, output)
                .map_err(|error| PyValueError::new_err(error.to_string()))
        })
    }
    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        PyArray1::from_vec(py, self.output.clone())
    }
    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
