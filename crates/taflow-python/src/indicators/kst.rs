use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::KnowSureThing;

#[pyclass]
pub struct KnowSureThingOperator {
    inner: KnowSureThing,
    kst: Vec<f64>,
    signal: Vec<f64>,
}

#[pymethods]
impl KnowSureThingOperator {
    #[new]
    #[pyo3(signature = (roc1=10, roc2=15, roc3=20, roc4=30, sma1=10, sma2=10, sma3=10, sma4=15, signal=9))]
    fn new(
        roc1: usize,
        roc2: usize,
        roc3: usize,
        roc4: usize,
        sma1: usize,
        sma2: usize,
        sma3: usize,
        sma4: usize,
        signal: usize,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: KnowSureThing::new(roc1, roc2, roc3, roc4, sma1, sma2, sma3, sma4, signal)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            kst: Vec::new(),
            signal: Vec::new(),
        })
    }

    fn append(&mut self, close: f64) -> (f64, f64) {
        let value = self.inner.append(close);
        self.kst.push(value.kst);
        self.signal.push(value.signal);
        (value.kst, value.signal)
    }

    fn extend(&mut self, py: Python<'_>, close: PyReadonlyArray1<f64>) -> PyResult<()> {
        let close = close.as_slice()?;
        py.allow_threads(|| {
            for &close in close {
                self.append(close);
            }
        });
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>) {
        (
            PyArray1::from_vec(py, self.kst.clone()),
            PyArray1::from_vec(py, self.signal.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64)> {
        self.inner.value().map(|value| (value.kst, value.signal))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.kst.clear();
        self.signal.clear();
    }
}
