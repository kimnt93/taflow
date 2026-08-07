use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::{HilbertTransformPhasor as NativeHilbertTransformPhasor, HtPhasorValue};

#[pyclass]
pub struct HilbertTransformPhasor {
    inner: NativeHilbertTransformPhasor,
    inphase: Vec<f64>,
    quadrature: Vec<f64>,
}

#[pymethods]
impl HilbertTransformPhasor {
    #[new]
    fn new() -> Self {
        Self {
            inner: NativeHilbertTransformPhasor::new(),
            inphase: Vec::new(),
            quadrature: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self.inner.append(input);
        let (inphase, quadrature) = value
            .map(|v| (v.inphase, v.quadrature))
            .unwrap_or((f64::NAN, f64::NAN));
        self.inphase.push(inphase);
        self.quadrature.push(quadrature);
        value.map(|v| (v.inphase, v.quadrature))
    }
    fn extend(&mut self, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        for &input in input.as_slice()? {
            self.append(input);
        }
        Ok(())
    }
    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.inphase.clone()),
            to_py_array(py, self.quadrature.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(
            |HtPhasorValue {
                 inphase,
                 quadrature,
             }| (inphase, quadrature),
        )
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.inphase.clear();
        self.quadrature.clear();
    }
}
