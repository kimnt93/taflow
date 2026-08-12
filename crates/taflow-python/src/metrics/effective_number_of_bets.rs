use numpy::{PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::EffectiveNumberOfBets as State, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

/// Native state backing `taflow.metrics.EffectiveNumberOfBets`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct EffectiveNumberOfBets {
    inner: State,
}

#[pymethods]
impl EffectiveNumberOfBets {
    #[new]
    #[pyo3(signature = (nan_policy="omit"))]
    fn new(nan_policy: &str) -> PyResult<Self> {
        Ok(Self {
            inner: State::new(NanPolicy::try_from(nan_policy).map_err(value_error)?)
                .map_err(value_error)?,
        })
    }

    #[pyo3(signature = (weights, covariance))]
    fn from_weights_and_covariance(
        &mut self,
        weights: PyReadonlyArray1<'_, f64>,
        covariance: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<()> {
        let weights = weights.as_slice()?;
        let covariance_shape = covariance.shape();
        if covariance_shape != [weights.len(), weights.len()] {
            return Err(PyValueError::new_err(
                "covariance must be square and match weights",
            ));
        }
        let covariance = covariance.as_slice()?;
        self.inner
            .from_weights_and_covariance(weights, covariance)
            .map(|_| ())
            .map_err(value_error)
    }

    fn from_risk_contributions(
        &mut self,
        py: Python<'_>,
        values: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.from_risk_contributions(values))
            .map(|_| ())
            .map_err(value_error)
    }

    fn append(&mut self, contribution: f64) -> PyResult<Option<f64>> {
        self.inner.append(contribution).map_err(value_error)
    }

    fn extend(&mut self, py: Python<'_>, values: PyReadonlyArray1<'_, f64>) -> PyResult<()> {
        let values = values.as_slice()?;
        py.allow_threads(|| self.inner.extend(values))
            .map(|_| ())
            .map_err(value_error)
    }

    #[getter]
    fn value(&self) -> Option<f64> {
        self.inner.value()
    }

    fn compute(&self) -> Option<f64> {
        self.inner.compute()
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }
}
