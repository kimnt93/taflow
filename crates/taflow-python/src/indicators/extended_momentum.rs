use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{AwesomeOscillator, FisherTransform, TrueStrengthIndex};
#[pyclass]
pub struct TrueStrengthIndexOperator {
    inner: TrueStrengthIndex,
    outputs: Vec<f64>,
}
#[pymethods]
impl TrueStrengthIndexOperator {
    #[new]
    fn new(fast: usize, slow: usize) -> PyResult<Self> {
        Ok(Self {
            inner: TrueStrengthIndex::new(fast, slow)
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
#[pyclass]
pub struct AwesomeOscillatorOperator {
    inner: AwesomeOscillator,
    outputs: Vec<f64>,
}
#[pymethods]
impl AwesomeOscillatorOperator {
    #[new]
    fn new(fast: usize, slow: usize) -> PyResult<Self> {
        Ok(Self {
            inner: AwesomeOscillator::new(fast, slow)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let v = self.inner.append(high, low);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (h, l) = (high.as_slice()?, low.as_slice()?);
        if h.len() != l.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&h, &l) in h.iter().zip(l) {
            self.append(h, l);
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
#[pyclass]
pub struct FisherTransformOperator {
    inner: FisherTransform,
    outputs: Vec<f64>,
}
#[pymethods]
impl FisherTransformOperator {
    #[new]
    fn new(timeperiod: usize) -> PyResult<Self> {
        Ok(Self {
            inner: FisherTransform::new(timeperiod)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
            outputs: Vec::new(),
        })
    }
    fn append(&mut self, high: f64, low: f64) -> Option<f64> {
        let v = self.inner.append(high, low);
        self.outputs.push(v.unwrap_or(f64::NAN));
        v
    }
    fn extend(&mut self, high: PyReadonlyArray1<f64>, low: PyReadonlyArray1<f64>) -> PyResult<()> {
        let (h, l) = (high.as_slice()?, low.as_slice()?);
        if h.len() != l.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        for (&h, &l) in h.iter().zip(l) {
            self.append(h, l);
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
