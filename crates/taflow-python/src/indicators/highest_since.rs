use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::HighestSince;

#[pyclass]
pub struct HighestSinceOperator {
    inner: HighestSince,
    outputs: Vec<f64>,
}

#[pymethods]
impl HighestSinceOperator {
    #[new]
    fn new() -> Self {
        Self {
            inner: HighestSince::new(),
            outputs: Vec::new(),
        }
    }
    fn append(&mut self, condition: bool, input: f64) -> Option<f64> {
        let value = self.inner.append(condition, input);
        self.outputs.push(value.unwrap_or(f64::NAN));
        value
    }
    fn extend(
        &mut self,
        py: Python<'_>,
        condition: PyReadonlyArray1<bool>,
        input: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (condition, input) = (condition.as_slice()?, input.as_slice()?);
        if condition.len() != input.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "inputs must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            condition.iter().zip(input).for_each(|(&c, &x)| {
                self.append(c, x);
            })
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
    fn __len__(&self) -> usize {
        self.outputs.len()
    }
}
