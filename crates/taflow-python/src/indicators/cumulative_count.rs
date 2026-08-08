use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::CumulativeCount;

/// Native persistent adapter for the one-based cumulative observation count.
#[pyclass]
pub struct CumulativeCountOperator {
    inner: CumulativeCount,
    outputs: Vec<f64>,
}

#[pymethods]
impl CumulativeCountOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: CumulativeCount::new(),
            outputs: Vec::new(),
        }
    }

    fn append(&mut self, input: f64) -> f64 {
        let value = self.inner.append(input);
        self.outputs.push(value);
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
        PyArray1::from_vec(py, self.outputs.clone())
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
