use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::RollingTreynorRatio as State;
#[pyclass]
pub struct RollingTreynorRatio {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl RollingTreynorRatio {
    #[new]
    #[pyo3(signature=(timeperiod=14))]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(timeperiod).map_err(|e| PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
        let v = self.inner.append(input, benchmark);
        self.output.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
        benchmark: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (input, benchmark) = (input.as_slice()?, benchmark.as_slice()?);
        if input.len() != benchmark.len() {
            return Err(PyValueError::new_err(
                "input and benchmark must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for (&x, &y) in input.iter().zip(benchmark) {
                self.append(x, y);
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
        self.output.clear()
    }
    fn __len__(&self) -> usize {
        self.output.len()
    }
}
