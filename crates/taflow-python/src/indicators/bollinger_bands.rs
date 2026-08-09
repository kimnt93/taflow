use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::{BollingerBands as State, StreamingIndicator};
use taflow::MaType;

#[pyclass]
pub struct BollingerBands {
    inner: State,
    upper: Vec<f64>,
    middle: Vec<f64>,
    lower: Vec<f64>,
}

#[pymethods]
impl BollingerBands {
    #[new]
    #[pyo3(signature = (timeperiod=5, nbdevup=2.0, nbdevdn=2.0, matype=0))]
    fn new(timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(
                timeperiod,
                nbdevup,
                nbdevdn,
                MaType::try_from(matype).map_err(|e| PyValueError::new_err(e.to_string()))?,
            )
            .map_err(|e| PyValueError::new_err(e.to_string()))?,
            upper: Vec::new(),
            middle: Vec::new(),
            lower: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self
            .inner
            .append(input)
            .map(|value| (value.upper, value.middle, value.lower));
        let output = value.unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.upper.push(output.0);
        self.middle.push(output.1);
        self.lower.push(output.2);
        value
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, &mut self.upper, &mut self.middle, &mut self.lower)
        });
        Ok(())
    }

    fn compute<'py>(
        &self,
        py: Python<'py>,
    ) -> (
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        (
            PyArray1::from_vec(py, self.upper.clone()),
            PyArray1::from_vec(py, self.middle.clone()),
            PyArray1::from_vec(py, self.lower.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.upper, value.middle, value.lower))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.upper.clear();
        self.middle.clear();
        self.lower.clear();
    }

    fn __len__(&self) -> usize {
        self.upper.len()
    }
}
