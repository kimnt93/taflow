/// Supported parameter default values.
#[derive(Debug, Clone)]
pub enum ParamDefault {
    Integer(i32),
    Real(f64),
    MaType(i32),
}

/// Definition of an indicator parameter.
#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: &'static str,
    pub default: ParamDefault,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Flags describing indicator behavior.
#[derive(Debug, Clone, Copy, Default)]
pub struct IndicatorFlags {
    /// Whether the indicator has an unstable warm-up period.
    pub has_unstable_period: bool,
    /// Whether the indicator recognizes candlestick patterns.
    pub is_candlestick: bool,
}

/// Indicator metadata used by the abstract API and introspection.
pub trait IndicatorInfo {
    /// Indicator name, for example `"SMA"`.
    fn name() -> &'static str;
    /// Indicator group, for example `"Overlap Studies"`.
    fn group() -> &'static str;
    /// Required input names, for example `["close"]` or
    /// `["high", "low", "close"]`.
    fn input_names() -> &'static [&'static str];
    /// Output names, for example `["real"]` or
    /// `["upperband", "middleband", "lowerband"]`.
    fn output_names() -> &'static [&'static str];
    /// Parameter definitions.
    fn parameters() -> Vec<ParamDef>;
    /// Indicator flags.
    fn flags() -> IndicatorFlags;
}
