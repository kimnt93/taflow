# taflow correctness verification

Date: 2026-08-07 | bars: 10,000 | warm-up split: 9,000 + 1,000 continue | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.5.1, TA-Lib 0.7.1, taflow 0.1.2

Summary: MATCH: 166, MISMATCH: 2

Columns — *batch vs oracle*: full-series batch against the
reference; *continue vs batch*: 9k `extend` + 1k `append` stitched
output bitwise-identical to the one-shot batch (chunk-invariance
contract); *continue vs oracle*: the stitched output against the
reference.

| Function | Oracle | Verdict | Batch vs oracle | Continue vs batch (bitwise) | Continue vs oracle |
|---|---|---|---|---|---|
| rolling_kurtosis | pandas | MISMATCH | **FAIL** (err 1.7e-04, nan 0) | yes | **FAIL** (err 1.7e-04, nan 0) |
| rolling_skew | pandas | MISMATCH | **FAIL** (err 3.8e-07, nan 0) | yes | **FAIL** (err 3.8e-07, nan 0) |
| ACCBANDS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ACOS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| AD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ADD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ADOSC | TA-Lib | MATCH | pass (err 1.5e-08, nan 0) | yes | pass (err 1.5e-08, nan 0) |
| ADX | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | pass (err 2.8e-14, nan 0) |
| ADXR | TA-Lib | MATCH | pass (err 2.1e-14, nan 0) | yes | pass (err 2.1e-14, nan 0) |
| APO | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| AROON | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| AROONOSC | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | pass (err 1.4e-14, nan 0) |
| ASIN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ATAN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ATR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| AVGDEV | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| AVGPRICE | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | pass (err 5.7e-14, nan 0) |
| BBANDS | TA-Lib | MATCH | pass (err 7.9e-10, nan 0) | yes | pass (err 7.9e-10, nan 0) |
| BETA | TA-Lib | MATCH | pass (err 5.7e-12, nan 0) | yes | pass (err 5.7e-12, nan 0) |
| BOP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| CCI | TA-Lib | MATCH | pass (err 2.6e-10, nan 0) | yes | pass (err 2.6e-10, nan 0) |
| CDL2CROWS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3BLACKCROWS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3INSIDE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3LINESTRIKE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3OUTSIDE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3STARSINSOUTH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDL3WHITESOLDIERS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLABANDONEDBABY | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLADVANCEBLOCK | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLBELTHOLD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLBREAKAWAY | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLCLOSINGMARUBOZU | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLCONCEALBABYSWALL | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLCOUNTERATTACK | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLDARKCLOUDCOVER | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLDOJI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLDOJISTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLDRAGONFLYDOJI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLENGULFING | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLEVENINGDOJISTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLEVENINGSTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLGAPSIDESIDEWHITE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLGRAVESTONEDOJI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHAMMER | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHANGINGMAN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHARAMI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHARAMICROSS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHIGHWAVE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHIKKAKE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHIKKAKEMOD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLHOMINGPIGEON | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLIDENTICAL3CROWS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLINNECK | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLINVERTEDHAMMER | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLKICKING | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLKICKINGBYLENGTH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLLADDERBOTTOM | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLLONGLEGGEDDOJI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLLONGLINE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLMARUBOZU | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLMATCHINGLOW | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLMATHOLD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLMORNINGDOJISTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLMORNINGSTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLONNECK | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLPIERCING | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLRICKSHAWMAN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLRISEFALL3METHODS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSEPARATINGLINES | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSHOOTINGSTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSHORTLINE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSPINNINGTOP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSTALLEDPATTERN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLSTICKSANDWICH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLTAKURI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLTASUKIGAP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLTHRUSTING | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLTRISTAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLUNIQUE3RIVER | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLUPSIDEGAP2CROWS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CDLXSIDEGAP3METHODS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| CEIL | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| CMO | TA-Lib | MATCH | pass (err 6.8e-14, nan 0) | yes | pass (err 6.8e-14, nan 0) |
| CORREL | TA-Lib | MATCH | pass (err 5.6e-11, nan 0) | yes | pass (err 5.6e-11, nan 0) |
| COS | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| COSH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| DEMA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| DIV | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| DX | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | pass (err 2.8e-14, nan 0) |
| EMA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| EXP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| FLOOR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| HT_DCPERIOD | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| HT_DCPHASE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| HT_PHASOR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| HT_SINE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| HT_TRENDLINE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| HT_TRENDMODE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| IMI | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | pass (err 1.4e-14, nan 0) |
| KAMA | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | pass (err 2.8e-14, nan 0) |
| LINEARREG | TA-Lib | MATCH | pass (err 6.0e-13, nan 0) | yes | pass (err 6.0e-13, nan 0) |
| LINEARREG_ANGLE | TA-Lib | MATCH | pass (err 4.9e-12, nan 0) | yes | pass (err 4.9e-12, nan 0) |
| LINEARREG_INTERCEPT | TA-Lib | MATCH | pass (err 6.0e-13, nan 0) | yes | pass (err 6.0e-13, nan 0) |
| LINEARREG_SLOPE | TA-Lib | MATCH | pass (err 9.1e-14, nan 0) | yes | pass (err 9.1e-14, nan 0) |
| LN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| LOG10 | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MACD | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | pass (err 1.4e-14, nan 0) |
| MACDEXT | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MACDFIX | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | pass (err 5.7e-14, nan 0) |
| MAMA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MAVP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MAX | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MAXINDEX | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MEDPRICE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MFI | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | — | — |
| MIDPOINT | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MIDPRICE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MIN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MININDEX | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MINMAX | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MINMAXINDEX | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MINUS_DI | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | — | — |
| MINUS_DM | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| MOM | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| MULT | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| NATR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| OBV | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| PLUS_DI | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | — | — |
| PLUS_DM | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | — | — |
| PPO | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ROC | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | pass (err 1.4e-14, nan 0) |
| ROCP | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ROCR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| ROCR100 | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| RSI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SAR | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SAREXT | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SIN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SINH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SMA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SQRT | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| STDDEV | TA-Lib | MATCH | pass (err 4.0e-10, nan 0) | yes | pass (err 4.0e-10, nan 0) |
| STOCH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| STOCHF | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| STOCHRSI | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SUB | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| SUM | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| TAN | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| TANH | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| TEMA | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| TRANGE | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| TRIMA | TA-Lib | MATCH | pass (err 6.8e-13, nan 0) | yes | pass (err 6.8e-13, nan 0) |
| TRIX | TA-Lib | MATCH | pass (err 1.1e-14, nan 0) | — | — |
| TSF | TA-Lib | MATCH | pass (err 6.8e-13, nan 0) | yes | pass (err 6.8e-13, nan 0) |
| TYPPRICE | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | pass (err 5.7e-14, nan 0) |
| ULTOSC | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | — | — |
| VAR | TA-Lib | MATCH | pass (err 3.1e-10, nan 0) | yes | pass (err 3.1e-10, nan 0) |
| WCLPRICE | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | pass (err 5.7e-14, nan 0) |
| WILLR | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | pass (err 2.8e-14, nan 0) |
| WMA | TA-Lib | MATCH | pass (err 1.9e-10, nan 0) | yes | pass (err 1.9e-10, nan 0) |
| ewm_std | pandas | MATCH | pass (err 7.8e-14, nan 0) | yes | pass (err 7.8e-14, nan 0) |
| ewm_var | pandas | MATCH | pass (err 1.9e-12, nan 0) | yes | pass (err 1.9e-12, nan 0) |
| rolling_cov | pandas | MATCH | pass (err 1.3e-11, nan 0) | yes | pass (err 1.3e-11, nan 0) |
| rolling_median | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| rolling_quantile | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | pass (err 0.0e+00, nan 0) |
| rolling_zscore | pandas | MATCH | pass (err 6.4e-10, nan 0) | yes | pass (err 6.4e-10, nan 0) |

## Follow-ups

- Mismatches: rolling_skew, rolling_kurtosis
- Errors: none
