use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::Liquidity;

#[pyclass]
pub struct LiquidityOperator {
    inner: Liquidity,
    liquidity: Vec<f64>,
    level: Vec<f64>,
    swept: Vec<f64>,
}

#[pymethods]
impl LiquidityOperator {
    #[new]
    #[pyo3(signature = (swing_length=50, range_percent=0.01))]
    fn new(swing_length: usize, range_percent: f64) -> PyResult<Self> {
        Ok(Self {
            inner: Liquidity::new(swing_length, range_percent)
                .map_err(|error| PyValueError::new_err(error.to_string()))?,
            liquidity: Vec::new(),
            level: Vec::new(),
            swept: Vec::new(),
        })
    }

    fn append(&mut self, high: f64, low: f64) -> (f64, f64, f64) {
        let value = self.inner.append(high, low, f64::NAN);
        self.liquidity.push(value.liquidity);
        self.level.push(value.level);
        self.swept.push(value.swept);
        (value.liquidity, value.level, value.swept)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        high: PyReadonlyArray1<f64>,
        low: PyReadonlyArray1<f64>,
    ) -> PyResult<()> {
        let (high, low) = (high.as_slice()?, low.as_slice()?);
        if high.len() != low.len() {
            return Err(PyValueError::new_err("inputs must have equal lengths"));
        }
        py.allow_threads(|| {
            for (&high, &low) in high.iter().zip(low) {
                self.append(high, low);
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
            PyArray1::from_vec(py, self.liquidity.clone()),
            PyArray1::from_vec(py, self.level.clone()),
            PyArray1::from_vec(py, self.swept.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.liquidity, value.level, value.swept))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.liquidity.clear();
        self.level.clear();
        self.swept.clear();
    }
}
