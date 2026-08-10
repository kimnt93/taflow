use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::{SessionRange as State, SessionRangeValue};

#[pyclass]
pub struct SessionRange {
    inner: State,
    asia: Vec<f64>,
    europe: Vec<f64>,
    united_states: Vec<f64>,
}

#[pymethods]
impl SessionRange {
    #[new]
    #[pyo3(signature=(utc_offset_minutes=0))]
    fn new(utc_offset_minutes: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(utc_offset_minutes)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            asia: Vec::new(),
            europe: Vec::new(),
            united_states: Vec::new(),
        })
    }

    fn append(
        &mut self,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        timestamp: i64,
    ) -> Option<(f64, f64, f64)> {
        let result = self.inner.append(open, high, low, close, volume, timestamp);
        let aligned = result.unwrap_or(SessionRangeValue {
            asia: f64::NAN,
            europe: f64::NAN,
            united_states: f64::NAN,
        });
        self.asia.push(aligned.asia);
        self.europe.push(aligned.europe);
        self.united_states.push(aligned.united_states);
        result.map(|value| (value.asia, value.europe, value.united_states))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        open: PyReadonlyArray1<f64>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
        timestamp: PyReadonlyArray1<i64>,
    ) -> PyResult<()> {
        let (open, high, low, close, volume, timestamp) = (
            open.as_slice()?,
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
            timestamp.as_slice()?,
        );
        if [
            open.len(),
            high.len(),
            low.len(),
            close.len(),
            volume.len(),
            timestamp.len(),
        ]
        .windows(2)
        .any(|lengths| lengths[0] != lengths[1])
        {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "OHLCV and timestamp must have equal lengths",
            ));
        }
        py.allow_threads(|| {
            for index in 0..open.len() {
                self.append(
                    open[index],
                    high[index],
                    low[index],
                    close[index],
                    volume[index],
                    timestamp[index],
                );
            }
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
            PyArray1::from_vec(py, self.asia.clone()),
            PyArray1::from_vec(py, self.europe.clone()),
            PyArray1::from_vec(py, self.united_states.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.asia, value.europe, value.united_states))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.asia.clear();
        self.europe.clear();
        self.united_states.clear();
    }

    fn __len__(&self) -> usize {
        self.asia.len()
    }
}
