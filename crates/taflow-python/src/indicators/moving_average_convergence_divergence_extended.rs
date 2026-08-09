use numpy::{PyArray1, PyReadonlyArray1};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use taflow::stream::MovingAverageConvergenceDivergenceExtended as State;
use taflow::MaType;

/// Native-backed Python boundary for canonical MACDEXT state.
#[pyclass]
pub struct MovingAverageConvergenceDivergenceExtended {
    inner: State,
    macds: Vec<f64>,
    signals: Vec<f64>,
    histograms: Vec<f64>,
}

#[pymethods]
impl MovingAverageConvergenceDivergenceExtended {
    #[new]
    #[pyo3(signature = (fastperiod=12, fastmatype=1, slowperiod=26, slowmatype=1, signalperiod=9, signalmatype=1))]
    fn new(
        fastperiod: usize,
        fastmatype: i32,
        slowperiod: usize,
        slowmatype: i32,
        signalperiod: usize,
        signalmatype: i32,
    ) -> PyResult<Self> {
        let fastmatype = MaType::try_from(fastmatype)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        let slowmatype = MaType::try_from(slowmatype)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        let signalmatype = MaType::try_from(signalmatype)
            .map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self {
            inner: State::new(
                fastperiod,
                fastmatype,
                slowperiod,
                slowmatype,
                signalperiod,
                signalmatype,
            )
            .map_err(|error| PyValueError::new_err(error.to_string()))?,
            macds: Vec::new(),
            signals: Vec::new(),
            histograms: Vec::new(),
        })
    }

    fn append(&mut self, input: f64) -> Option<(f64, f64, f64)> {
        let value = self.inner.append(input);
        let output = value
            .map(|value| (value.macd, value.signal, value.histogram))
            .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
        self.macds.push(output.0);
        self.signals.push(output.1);
        self.histograms.push(output.2);
        value.map(|value| (value.macd, value.signal, value.histogram))
    }

    fn extend(&mut self, py: Python<'_>, input: PyReadonlyArray1<f64>) -> PyResult<()> {
        let input = input.as_slice()?;
        let (macds, signals, histograms) =
            (&mut self.macds, &mut self.signals, &mut self.histograms);
        py.allow_threads(|| {
            self.inner
                .extend_slices_into(input, macds, signals, histograms)
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
            PyArray1::from_vec(py, self.macds.clone()),
            PyArray1::from_vec(py, self.signals.clone()),
            PyArray1::from_vec(py, self.histograms.clone()),
        )
    }

    #[getter]
    fn value(&self) -> Option<(f64, f64, f64)> {
        self.inner
            .value()
            .map(|value| (value.macd, value.signal, value.histogram))
    }

    fn reset(&mut self) {
        self.inner.reset();
        self.macds.clear();
        self.signals.clear();
        self.histograms.clear();
    }

    fn __len__(&self) -> usize {
        self.macds.len()
    }
}
