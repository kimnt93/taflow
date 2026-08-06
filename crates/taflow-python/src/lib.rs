#![allow(non_snake_case)]

use pyo3::prelude::*;

mod conversion;
mod func_api;
mod indicators;
mod metadata;
mod state_api;

/// Python module entry point for `taflow._native`.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 元数据 API
    m.add_function(wrap_pyfunction!(metadata::get_functions, m)?)?;
    m.add_function(wrap_pyfunction!(metadata::get_function_groups, m)?)?;

    // Unified indicator objects. Each TA binding lives in its own module and
    // owns its accumulated outputs so compute() never replays prior input.
    m.add_class::<indicators::ExponentialMovingAverage>()?;
    m.add_class::<indicators::CommodityChannelIndex>()?;
    m.add_class::<indicators::MoneyFlowIndex>()?;
    m.add_class::<indicators::PlusDirectionalIndicator>()?;
    m.add_class::<indicators::PlusDirectionalMovement>()?;
    m.add_class::<indicators::MinusDirectionalIndicator>()?;
    m.add_class::<indicators::MinusDirectionalMovement>()?;
    m.add_class::<indicators::TripleExponentialRateOfChange>()?;
    m.add_class::<indicators::UltimateOscillator>()?;

    // Transitional state classes remain available while their indicators move
    // to the unified object surface above.
    m.add_class::<state_api::StatefulSma>()?;
    m.add_class::<state_api::StatefulEma>()?;
    m.add_class::<state_api::StatefulWma>()?;
    m.add_class::<state_api::StatefulDema>()?;
    m.add_class::<state_api::StatefulTema>()?;
    m.add_class::<state_api::StatefulTrima>()?;
    m.add_class::<state_api::StatefulMidpoint>()?;
    m.add_class::<state_api::StatefulMidprice>()?;
    m.add_class::<state_api::StatefulRsi>()?;
    m.add_class::<state_api::StatefulImi>()?;
    m.add_class::<state_api::StatefulMom>()?;
    m.add_class::<state_api::StatefulRoc>()?;
    m.add_class::<state_api::StatefulRocp>()?;
    m.add_class::<state_api::StatefulRocr>()?;
    m.add_class::<state_api::StatefulRocr100>()?;
    m.add_class::<state_api::StatefulMax>()?;
    m.add_class::<state_api::StatefulMaxindex>()?;
    m.add_class::<state_api::StatefulMin>()?;
    m.add_class::<state_api::StatefulMinindex>()?;
    m.add_class::<state_api::StatefulSum>()?;
    m.add_class::<state_api::StatefulMinmax>()?;
    m.add_class::<state_api::StatefulMinmaxindex>()?;
    m.add_class::<state_api::StatefulAvgdev>()?;
    m.add_class::<state_api::StatefulVar>()?;
    m.add_class::<state_api::StatefulStddev>()?;
    m.add_class::<state_api::StatefulBeta>()?;
    m.add_class::<state_api::StatefulCorrel>()?;
    m.add_class::<state_api::StatefulLinearreg>()?;
    m.add_class::<state_api::StatefulLinearregSlope>()?;
    m.add_class::<state_api::StatefulLinearregIntercept>()?;
    m.add_class::<state_api::StatefulLinearregAngle>()?;
    m.add_class::<state_api::StatefulTsf>()?;
    m.add_class::<state_api::StatefulCmo>()?;
    m.add_class::<state_api::StatefulCci>()?;
    m.add_class::<state_api::StatefulKama>()?;
    m.add_class::<state_api::StatefulAd>()?;
    m.add_class::<state_api::StatefulAdosc>()?;
    m.add_class::<state_api::StatefulObv>()?;
    m.add_class::<state_api::StatefulBop>()?;
    m.add_class::<state_api::StatefulWillr>()?;
    m.add_class::<state_api::StatefulAroon>()?;
    m.add_class::<state_api::StatefulAroonosc>()?;
    m.add_class::<state_api::StatefulAvgprice>()?;
    m.add_class::<state_api::StatefulMedprice>()?;
    m.add_class::<state_api::StatefulTypprice>()?;
    m.add_class::<state_api::StatefulWclprice>()?;
    m.add_class::<state_api::StatefulAcos>()?;
    m.add_class::<state_api::StatefulAsin>()?;
    m.add_class::<state_api::StatefulAtan>()?;
    m.add_class::<state_api::StatefulCeil>()?;
    m.add_class::<state_api::StatefulCos>()?;
    m.add_class::<state_api::StatefulCosh>()?;
    m.add_class::<state_api::StatefulExp>()?;
    m.add_class::<state_api::StatefulFloor>()?;
    m.add_class::<state_api::StatefulLn>()?;
    m.add_class::<state_api::StatefulLog10>()?;
    m.add_class::<state_api::StatefulSin>()?;
    m.add_class::<state_api::StatefulSinh>()?;
    m.add_class::<state_api::StatefulSqrt>()?;
    m.add_class::<state_api::StatefulTan>()?;
    m.add_class::<state_api::StatefulTanh>()?;
    m.add_class::<state_api::StatefulAdd>()?;
    m.add_class::<state_api::StatefulSub>()?;
    m.add_class::<state_api::StatefulMult>()?;
    m.add_class::<state_api::StatefulDiv>()?;
    m.add_class::<state_api::StatefulAtr>()?;
    m.add_class::<state_api::StatefulTrange>()?;
    m.add_class::<state_api::StatefulNatr>()?;
    m.add_class::<state_api::StatefulMacd>()?;
    m.add_class::<state_api::StatefulMacdExt>()?;
    m.add_class::<state_api::StatefulMavp>()?;
    m.add_class::<state_api::StatefulHtTrendline>()?;
    m.add_class::<state_api::StatefulAdx>()?;
    m.add_class::<state_api::StatefulAdxr>()?;
    m.add_class::<state_api::StatefulDx>()?;
    m.add_class::<state_api::StatefulMacdFix>()?;
    m.add_class::<state_api::StatefulStoch>()?;
    m.add_class::<state_api::StatefulStochf>()?;
    m.add_class::<state_api::StatefulStochrsi>()?;
    m.add_class::<state_api::StatefulMama>()?;
    m.add_class::<state_api::StatefulT3>()?;
    m.add_class::<state_api::StatefulApo>()?;
    m.add_class::<state_api::StatefulPpo>()?;
    m.add_class::<state_api::StatefulMa>()?;
    m.add_class::<state_api::StatefulBbands>()?;
    m.add_class::<state_api::StatefulAccbands>()?;
    m.add_class::<state_api::StatefulSar>()?;
    m.add_class::<state_api::StatefulSarext>()?;

    // ===== Overlap Studies =====
    m.add_function(wrap_pyfunction!(func_api::ACCBANDS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::EMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::WMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::DEMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TEMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TRIMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::KAMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::T3, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MAMA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::BBANDS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SAREXT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MIDPOINT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MIDPRICE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MAVP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::HT_TRENDLINE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MA, m)?)?;

    // ===== Momentum Indicators =====
    m.add_function(wrap_pyfunction!(func_api::IMI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::RSI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MACD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MACDEXT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MACDFIX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::STOCH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::STOCHF, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::STOCHRSI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ADX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ADXR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CCI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MOM, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ROC, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ROCP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ROCR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ROCR100, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::WILLR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::APO, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::PPO, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::BOP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CMO, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::AROON, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::AROONOSC, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MFI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TRIX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ULTOSC, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::DX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::PLUS_DI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MINUS_DI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::PLUS_DM, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MINUS_DM, m)?)?;

    // ===== Volatility =====
    m.add_function(wrap_pyfunction!(func_api::ATR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::NATR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TRANGE, m)?)?;

    // ===== Volume =====
    m.add_function(wrap_pyfunction!(func_api::AD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ADOSC, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::OBV, m)?)?;

    // ===== Price Transform =====
    m.add_function(wrap_pyfunction!(func_api::AVGPRICE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MEDPRICE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TYPPRICE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::WCLPRICE, m)?)?;

    // ===== Statistic =====
    m.add_function(wrap_pyfunction!(func_api::AVGDEV, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::STDDEV, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::VAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::BETA, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CORREL, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LINEARREG, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LINEARREG_SLOPE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LINEARREG_INTERCEPT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LINEARREG_ANGLE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TSF, m)?)?;

    // ===== Math Transform =====
    m.add_function(wrap_pyfunction!(func_api::ACOS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ASIN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::ATAN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CEIL, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::COS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::COSH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::EXP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::FLOOR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::LOG10, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SIN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SINH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SQRT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TAN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::TANH, m)?)?;

    // ===== Math Operators =====
    m.add_function(wrap_pyfunction!(func_api::ADD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SUB, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MULT, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::DIV, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MAX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MAXINDEX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MIN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MININDEX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::SUM, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MINMAX, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::MINMAXINDEX, m)?)?;

    // ===== Cycle Indicators =====
    m.add_function(wrap_pyfunction!(func_api::HT_DCPERIOD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::HT_DCPHASE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::HT_PHASOR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::HT_SINE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::HT_TRENDMODE, m)?)?;

    // ===== Pattern Recognition =====
    m.add_function(wrap_pyfunction!(func_api::CDLDOJI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHAMMER, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLENGULFING, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL2CROWS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3BLACKCROWS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3INSIDE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3LINESTRIKE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3OUTSIDE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3STARSINSOUTH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDL3WHITESOLDIERS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLABANDONEDBABY, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLADVANCEBLOCK, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLBELTHOLD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLBREAKAWAY, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLCLOSINGMARUBOZU, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLCONCEALBABYSWALL, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLCOUNTERATTACK, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLDARKCLOUDCOVER, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLDOJISTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLDRAGONFLYDOJI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLEVENINGDOJISTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLEVENINGSTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLGAPSIDESIDEWHITE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLGRAVESTONEDOJI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHANGINGMAN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHARAMI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHARAMICROSS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHIGHWAVE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHIKKAKE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHIKKAKEMOD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLHOMINGPIGEON, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLIDENTICAL3CROWS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLINNECK, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLINVERTEDHAMMER, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLKICKING, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLKICKINGBYLENGTH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLLADDERBOTTOM, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLLONGLEGGEDDOJI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLLONGLINE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLMARUBOZU, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLMATCHINGLOW, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLMATHOLD, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLMORNINGDOJISTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLMORNINGSTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLONNECK, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLPIERCING, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLRICKSHAWMAN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLRISEFALL3METHODS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSEPARATINGLINES, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSHOOTINGSTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSHORTLINE, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSPINNINGTOP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSTALLEDPATTERN, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLSTICKSANDWICH, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLTAKURI, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLTASUKIGAP, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLTHRUSTING, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLTRISTAR, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLUNIQUE3RIVER, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLUPSIDEGAP2CROWS, m)?)?;
    m.add_function(wrap_pyfunction!(func_api::CDLXSIDEGAP3METHODS, m)?)?;

    Ok(())
}
