use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{MovingAverage as MovingAverageState, StreamingIndicator};
use taflow::MaType;

/// Python boundary for the canonical Rust selectable moving-average state.
#[pyclass]
pub struct MovingAverage {
    inner: MovingAverageState,
    output: Vec<f64>,
}

#[pymethods]
impl MovingAverage {
    #[new]
    #[pyo3(signature = (timeperiod=30, matype=0))]
    fn new(timeperiod: usize, matype: i32) -> PyResult<Self> {
        let ma_type =
            MaType::try_from(matype).map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self {
            inner: MovingAverageState::new(timeperiod, ma_type)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
        })
    }

    fn append(&mut self, value: f64) -> Option<f64> {
        let result = self.inner.append(value);
        self.output.push(result.unwrap_or(f64::NAN));
        result
    }

    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        let inner = &mut self.inner;
        let output = &mut self.output;
        py.allow_threads(|| inner.extend_slice_into(values, output));
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
