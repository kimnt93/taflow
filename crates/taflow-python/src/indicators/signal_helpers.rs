use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{Cross, Crossover, Crossunder, Falling, Rising};
macro_rules! binary {
    ($name:ident,$inner:ty) => {
        #[pyclass]
        pub struct $name {
            inner: $inner,
            outputs: Vec<f64>,
        }
        #[pymethods]
        impl $name {
            #[new]
            fn new() -> Self {
                Self {
                    inner: <$inner>::new(),
                    outputs: Vec::new(),
                }
            }
            fn append(&mut self, left: f64, right: f64) -> f64 {
                let v = self.inner.append(left, right);
                self.outputs.push(v);
                v
            }
            fn extend(
                &mut self,
                left: PyReadonlyArray1<f64>,
                right: PyReadonlyArray1<f64>,
            ) -> PyResult<()> {
                let (a, b) = (left.as_slice()?, right.as_slice()?);
                if a.len() != b.len() {
                    return Err(PyValueError::new_err("inputs must have equal lengths"));
                }
                for (&x, &y) in a.iter().zip(b) {
                    self.append(x, y);
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
binary!(CrossoverOperator, Crossover);
binary!(CrossunderOperator, Crossunder);
binary!(CrossOperator, Cross);

macro_rules! unary {
    ($name:ident,$inner:ty) => {
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
                        .map_err(|e| PyValueError::new_err(e.to_string()))?,
                    outputs: Vec::new(),
                })
            }
            fn append(&mut self, input: f64) -> Option<f64> {
                let v = self.inner.append(input);
                self.outputs.push(v.unwrap_or(f64::NAN));
                v
            }
            fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
                for &v in input.as_slice()? {
                    self.append(v);
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
unary!(RisingOperator, Rising);
unary!(FallingOperator, Falling);
