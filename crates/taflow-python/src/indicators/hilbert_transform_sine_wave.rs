use crate::conversion::to_py_array;
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{
    HilbertTransformSineWave as NativeHilbertTransformSineWave, HilbertTransformSineWaveValue,
};

#[pyclass]
pub struct HilbertTransformSineWave {
    inner: NativeHilbertTransformSineWave,
    sine: Vec<f64>,
    leadsine: Vec<f64>,
}

#[pymethods]
impl HilbertTransformSineWave {
    #[new]
    fn new() -> Self {
        Self {
            inner: NativeHilbertTransformSineWave::new(),
            sine: Vec::new(),
            leadsine: Vec::new(),
        }
    }
    fn append(&mut self, input: f64) -> Option<(f64, f64)> {
        let value = self.inner.append(input);
        let (sine, leadsine) = value
            .map(|v| (v.sine, v.leadsine))
            .unwrap_or((f64::NAN, f64::NAN));
        self.sine.push(sine);
        self.leadsine.push(leadsine);
        value.map(|v| (v.sine, v.leadsine))
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
    fn compute(&self, py: Python<'_>) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>) {
        (
            to_py_array(py, self.sine.clone()),
            to_py_array(py, self.leadsine.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner
            .value()
            .map(|HilbertTransformSineWaveValue { sine, leadsine }| (sine, leadsine))
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.sine.clear();
        self.leadsine.clear();
    }
}
