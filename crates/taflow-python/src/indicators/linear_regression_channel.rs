use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::LinearRegressionChannel as State;
#[pyclass]
pub struct LinearRegressionChannel {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl LinearRegressionChannel {
    #[new]
    fn new(period: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, x: f64) -> Option<f64> {
        let v = self.inner.append(x);
        self.output.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, py: Python<'_>, x: PyReadonlyArray1<f64>) -> PyResult<()> {
        let x = x.as_slice()?;
        py.allow_threads(|| {
            for &a in x {
                self.append(a);
            }
        });
        Ok(())
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
