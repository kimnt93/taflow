use numpy::PyReadonlyArray1;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow_metrics::{metrics::TrackingError as State, MetricInputKind, NanPolicy};

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

fn input_kinds(
    name: &str,
    initial_equity: Option<f64>,
    benchmark_initial_equity: Option<f64>,
) -> PyResult<(MetricInputKind, MetricInputKind)> {
    match (name, initial_equity, benchmark_initial_equity) {
        ("returns", None, None) => Ok((MetricInputKind::Returns, MetricInputKind::Returns)),
        ("log_returns", None, None) => {
            Ok((MetricInputKind::LogReturns, MetricInputKind::LogReturns))
        }
        ("equity", None, None) => Ok((MetricInputKind::Equity, MetricInputKind::Equity)),
        ("pnl", Some(initial_equity), Some(benchmark_initial_equity)) => Ok((
            MetricInputKind::PeriodPnl { initial_equity },
            MetricInputKind::PeriodPnl {
                initial_equity: benchmark_initial_equity,
            },
        )),
        ("pnl", _, _) => Err(PyValueError::new_err(
            "initial_equity and benchmark_initial_equity are required for period-P&L conversion",
        )),
        (_, Some(_), _) | (_, _, Some(_)) => Err(PyValueError::new_err(
            "initial equity is accepted only for period-P&L input",
        )),
        _ => Err(PyValueError::new_err(
            "unsupported TrackingError input mode",
        )),
    }
}

/// Native state backing `taflow.metrics.TrackingError`.
#[pyclass(module = "taflow._native.metrics")]
pub(crate) struct TrackingError {
    inner: State,
}

#[pymethods]
impl TrackingError {
    #[new]
    #[pyo3(signature = (input_mode, periods_per_year=252.0, annualized=true, initial_equity=None, benchmark_initial_equity=None, nan_policy="omit"))]
    fn new(
        input_mode: &str,
        periods_per_year: f64,
        annualized: bool,
        initial_equity: Option<f64>,
        benchmark_initial_equity: Option<f64>,
        nan_policy: &str,
    ) -> PyResult<Self> {
        let (primary_input_kind, benchmark_input_kind) =
            input_kinds(input_mode, initial_equity, benchmark_initial_equity)?;
        Ok(Self {
            inner: State::new(
                primary_input_kind,
                benchmark_input_kind,
                periods_per_year,
                annualized,
                NanPolicy::try_from(nan_policy).map_err(value_error)?,
            )
            .map_err(value_error)?,
        })
    }

    fn append(&mut self, primary: f64, benchmark: f64) -> PyResult<Option<f64>> {
        self.inner.append(primary, benchmark).map_err(value_error)
    }

    fn extend(
        &mut self,
        py: Python<'_>,
        primary: PyReadonlyArray1<'_, f64>,
        benchmark: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<()> {
        let primary = primary.as_slice()?;
        let benchmark = benchmark.as_slice()?;
        py.allow_threads(|| self.inner.extend(primary, benchmark))
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
