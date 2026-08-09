use crate::conversion::to_py_array;
use crate::state_api::{extend_from_options, push_option, py_value_error};
use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::indicators;
use taflow::indicators::VariablePeriodMovingAverage as State;
use taflow::MaType;

#[pyclass]
pub struct VariablePeriodMovingAverage {
    inner: State,
    outputs: Vec<f64>,
}

#[pymethods]
impl VariablePeriodMovingAverage {
    #[new]
    #[pyo3(signature = (minperiod=2, maxperiod=30, matype=0))]
    fn new(minperiod: usize, maxperiod: usize, matype: i32) -> PyResult<Self> {
        let ma_type = MaType::try_from(matype).map_err(py_value_error)?;
        Ok(Self {
            inner: State::new(minperiod, maxperiod, ma_type).map_err(py_value_error)?,
            outputs: Vec::new(),
        })
    }

    fn append(&mut self, input: f64, period: f64) -> Option<f64> {
        push_option(&mut self.outputs, self.inner.append(input, period))
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        input: PyReadonlyArray1<f64>,
        periods: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let input = input.as_slice()?;
        let periods = periods.as_slice()?;
        if input.len() != periods.len() {
            return Err(PyValueError::new_err(
                "input and periods must have equal lengths",
            ));
        }
        let inner = &mut self.inner;
        let outputs = &mut self.outputs;
        py.allow_threads(|| inner.extend_slices_into(input, periods, outputs))
            .map_err(py_value_error)?;
        Ok(())
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self, py: Python<'_>) -> Py<PyArray1<f64>> {
        to_py_array(py, self.outputs.clone())
    }

    fn __len__(&self) -> usize {
        self.outputs.len()
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.outputs.clear();
    }
}
