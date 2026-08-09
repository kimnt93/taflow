use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;
use taflow::stream::{SessionVolumeLevels as State, SessionVolumeLevelsValue};

#[pyclass]
pub struct SessionVolumeLevels {
    inner: State,
    point_of_control: Vec<f64>,
    value_area_high: Vec<f64>,
    value_area_low: Vec<f64>,
}

#[pymethods]
impl SessionVolumeLevels {
    #[new]
    #[pyo3(signature = (bins=24, value_area=0.7))]
    fn new(bins: usize, value_area: f64) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(bins, value_area)
                .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))?,
            point_of_control: Vec::new(),
            value_area_high: Vec::new(),
            value_area_low: Vec::new(),
        })
    }
    fn append(
        &mut self,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        anchor: bool,
    ) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, close, volume, anchor);
        self.point_of_control.push(value.point_of_control);
        self.value_area_high.push(value.value_area_high);
        self.value_area_low.push(value.value_area_low);
        (
            value.point_of_control,
            value.value_area_high,
            value.value_area_low,
        )
    }
    fn extend(
        &mut self,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
        close: PyReadonlyArray1<f64>,
        volume: PyReadonlyArray1<f64>,
        anchor: PyReadonlyArray1<bool>,
    ) -> PyResult<()> {
        let (high, low, close, volume, anchor) = (
            high.as_slice()?,
            low.as_slice()?,
            close.as_slice()?,
            volume.as_slice()?,
            anchor.as_slice()?,
        );
        self.inner
            .extend_slice_into(
                high,
                low,
                close,
                volume,
                anchor,
                &mut self.point_of_control,
                &mut self.value_area_high,
                &mut self.value_area_low,
            )
            .map_err(|error| pyo3::exceptions::PyValueError::new_err(error.to_string()))
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
            PyArray1::from_vec(py, self.point_of_control.clone()),
            PyArray1::from_vec(py, self.value_area_high.clone()),
            PyArray1::from_vec(py, self.value_area_low.clone()),
        )
    }
    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner.value().map(|value: SessionVolumeLevelsValue| {
            (
                value.point_of_control,
                value.value_area_high,
                value.value_area_low,
            )
        })
    }
    fn reset(&mut self) {
        self.inner.reset();
        self.point_of_control.clear();
        self.value_area_high.clear();
        self.value_area_low.clear();
    }
    fn __len__(&self) -> usize {
        self.point_of_control.len()
    }
}
