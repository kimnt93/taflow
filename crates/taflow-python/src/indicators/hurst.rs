use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{FractalDimension, Hurst};

macro_rules! hurst_operator {
    ($name:ident, $inner:ty) => {
        #[pyclass]
        pub struct $name {
            inner: $inner,
            outputs: Vec<f64>,
        }
        #[pymethods]
        impl $name {
            #[new]
            fn new(timeperiod: usize) -> PyResult<Self> {
                Ok(Self {
                    inner: <$inner>::new(timeperiod)
                        .map_err(|error| PyValueError::new_err(error.to_string()))?,
                    outputs: Vec::new(),
                })
            }
            fn append(&mut self, input: f64) -> Option<f64> {
                let value = self.inner.append(input);
                self.outputs.push(value.unwrap_or(f64::NAN));
                value
            }
            fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                let input = input.as_slice()?;
                py.allow_threads(|| {
                    for &input in input {
                        self.append(input);
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
    };
}

hurst_operator!(HurstOperator, Hurst);
