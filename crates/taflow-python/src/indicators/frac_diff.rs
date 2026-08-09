use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators::FracDiff;

#[pyclass]
pub struct FracDiffOperator {
    inner: FracDiff,
    output: Vec<f64>,
}

#[pymethods]
impl FracDiffOperator {
    #[new]
    #[pyo3(signature = (d=0.5, threshold=1e-5))]
    fn new(d: f64, threshold: f64) -> PyResult<Self> {
        Ok(Self {
            inner: FracDiff::new(d, threshold)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<f64> {
        let value = self.inner.append(input);
        self.output.push(value.unwrap_or(f64::NAN));
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            for &value in input {
                self.append(value);
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
}
