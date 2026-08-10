use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::EmpiricalModeDecomposition as State;
#[pyclass]
pub struct EmpiricalModeDecomposition {
    inner: State,
    output: Vec<f64>,
}
#[pymethods]
impl EmpiricalModeDecomposition {
    #[new]
    #[pyo3(signature=(period=20,fraction=0.5))]
    fn new(period: usize, fraction: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(period, fraction)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?,
            output: Vec::new(),
        })
    }
    fn append(&mut self, value: f64) -> Option<f64> {
        let x = self.inner.append(value);
        self.output.push(x.unwrap_or(f64::NAN));
        x
    }
    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| {
            for &x in values {
                self.append(x);
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
