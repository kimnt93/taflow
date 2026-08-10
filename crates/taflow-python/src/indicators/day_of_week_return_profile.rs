use numpy::{ndarray::Array2, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::indicators::DayOfWeekReturnProfile as State;

const WEEKDAYS: usize = 7;

#[pyclass]
pub struct DayOfWeekReturnProfile {
    inner: State,
    output: Vec<f64>,
}

#[pymethods]
impl DayOfWeekReturnProfile {
    #[new]
    #[pyo3(signature=(utc_offset_minutes=0))]
    fn new(utc_offset_minutes: i32) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(utc_offset_minutes)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            output: Vec::new(),
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
    ) -> Option<Vec<f64>> {
        let row = self
            .inner
            .append(open, high, low, close, volume, timestamp)
            .map(|value| value.bins.to_vec());
        match &row {
            Some(values) => self.output.extend_from_slice(values),
            None => self.output.resize(self.output.len() + WEEKDAYS, f64::NAN),
        }
        row
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
                "OHLCV and timestamp inputs must have equal lengths",
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

    fn compute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f64>> {
        let rows = self.output.len() / WEEKDAYS;
        let array = Array2::from_shape_vec((rows, WEEKDAYS), self.output.clone())
            .expect("profile history has seven weekday columns");
        PyArray2::from_owned_array(py, array)
    }

    #[getter]
    fn value(&self) -> Option<Vec<f64>> {
        self.inner.value().map(|value| value.bins.to_vec())
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.output.clear();
    }

    fn __len__(&self) -> usize {
        self.output.len() / WEEKDAYS
    }
}
