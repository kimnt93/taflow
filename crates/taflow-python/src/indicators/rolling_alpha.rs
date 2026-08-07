use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{RollingAlpha, RollingInformationRatio};

macro_rules! paired {
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
            fn append(&mut self, input: f64, benchmark: f64) -> Option<f64> {
                let value = self.inner.append(input, benchmark);
                self.outputs.push(value.unwrap_or(f64::NAN));
                value
            }
            fn extend(
                &mut self,
                input: PyReadonlyArray1<f64>,
                benchmark: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let (input, benchmark) = (input.as_slice()?, benchmark.as_slice()?);
                if input.len() != benchmark.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                for (&input, &benchmark) in input.iter().zip(benchmark) {
                    self.append(input, benchmark);
                }
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

paired!(RollingAlphaOperator, RollingAlpha);
paired!(RollingInformationRatioOperator, RollingInformationRatio);
