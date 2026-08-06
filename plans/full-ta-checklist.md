# Full unified TA-Lib checklist

This is the authoritative inventory for the unified TAFlow indicator API. The
existing check marks are inherited core-state progress and must not be treated
as completion of the new public surface. During the migration, every item is
pending until it has its persistent Rust state, optimized
bulk-initialization/extend path, scalar continuation, Python class,
descriptive public name, and uppercase TA-Lib alias. The former separate batch
compatibility implementation is not a completion criterion: `taflow.talib`
will forward to this same stateful surface.

Complete the whole inventory before the exhaustive external TA-Lib comparison,
benchmark, and per-function report pass. During implementation, use focused
lifecycle tests (construction, `append`, `extend`, `compute`, and `reset`) but
do not delay a checklist item for its final oracle report. A check must never
mean merely that an old batch function exists.

This delivery is intentionally limited to TA-Lib. The separate operator
checklist is deferred and is not linked to this implementation gate.

## Overlap Studies

- [x] ACCBANDS
- [x] SMA
- [x] EMA
- [x] WMA
- [x] DEMA
- [x] TEMA
- [x] TRIMA
- [x] KAMA
- [x] T3
- [x] MAMA
- [x] BBANDS
- [x] SAR
- [x] SAREXT
- [x] MIDPOINT
- [x] MIDPRICE
- [x] MAVP
- [x] HT_TRENDLINE
- [x] MA

## Momentum Indicators

- [x] IMI
- [x] RSI
- [x] MACD
- [x] MACDEXT
- [x] MACDFIX
- [x] STOCH
- [x] STOCHF
- [x] STOCHRSI
- [x] ADX
- [x] ADXR
- [x] CCI
- [x] MOM
- [x] ROC
- [x] ROCP
- [x] ROCR
- [x] ROCR100
- [x] WILLR
- [x] APO
- [x] PPO
- [x] BOP
- [x] CMO
- [x] AROON
- [x] AROONOSC
- [x] MFI
- [x] TRIX
- [x] ULTOSC
- [x] DX
- [x] PLUS_DI
- [x] MINUS_DI
- [x] PLUS_DM
- [x] MINUS_DM

## Volatility Indicators

- [x] ATR
- [x] NATR
- [x] TRANGE

## Volume Indicators

- [x] AD
- [x] ADOSC
- [x] OBV

## Price Transform

- [x] AVGPRICE
- [x] MEDPRICE
- [x] TYPPRICE
- [x] WCLPRICE

## Statistic Functions

- [x] AVGDEV
- [x] STDDEV
- [x] VAR
- [x] BETA
- [x] CORREL
- [x] LINEARREG
- [x] LINEARREG_SLOPE
- [x] LINEARREG_INTERCEPT
- [x] LINEARREG_ANGLE
- [x] TSF

## Math Transform

- [x] ACOS
- [x] ASIN
- [x] ATAN
- [x] CEIL
- [x] COS
- [x] COSH
- [x] EXP
- [x] FLOOR
- [x] LN
- [x] LOG10
- [x] SIN
- [x] SINH
- [x] SQRT
- [x] TAN
- [x] TANH

## Math Operators

- [x] ADD
- [x] SUB
- [x] MULT
- [x] DIV
- [x] MAX
- [x] MAXINDEX
- [x] MIN
- [x] MININDEX
- [x] SUM
- [x] MINMAX
- [x] MINMAXINDEX

## Cycle Indicators

- [x] HT_DCPERIOD
- [x] HT_DCPHASE
- [x] HT_PHASOR
- [x] HT_SINE
- [x] HT_TRENDMODE

## Pattern Recognition

- [x] CDLDOJI
- [x] CDLHAMMER
- [x] CDLENGULFING
- [x] CDL2CROWS
- [ ] CDL3BLACKCROWS
- [ ] CDL3INSIDE
- [ ] CDL3LINESTRIKE
- [ ] CDL3OUTSIDE
- [ ] CDL3STARSINSOUTH
- [ ] CDL3WHITESOLDIERS
- [ ] CDLABANDONEDBABY
- [ ] CDLADVANCEBLOCK
- [ ] CDLBELTHOLD
- [ ] CDLBREAKAWAY
- [ ] CDLCLOSINGMARUBOZU
- [ ] CDLCONCEALBABYSWALL
- [ ] CDLCOUNTERATTACK
- [ ] CDLDARKCLOUDCOVER
- [ ] CDLDOJISTAR
- [ ] CDLDRAGONFLYDOJI
- [ ] CDLEVENINGDOJISTAR
- [ ] CDLEVENINGSTAR
- [ ] CDLGAPSIDESIDEWHITE
- [ ] CDLGRAVESTONEDOJI
- [ ] CDLHANGINGMAN
- [ ] CDLHARAMI
- [ ] CDLHARAMICROSS
- [ ] CDLHIGHWAVE
- [ ] CDLHIKKAKE
- [ ] CDLHIKKAKEMOD
- [ ] CDLHOMINGPIGEON
- [ ] CDLIDENTICAL3CROWS
- [ ] CDLINNECK
- [ ] CDLINVERTEDHAMMER
- [ ] CDLKICKING
- [ ] CDLKICKINGBYLENGTH
- [ ] CDLLADDERBOTTOM
- [ ] CDLLONGLEGGEDDOJI
- [ ] CDLLONGLINE
- [ ] CDLMARUBOZU
- [ ] CDLMATCHINGLOW
- [ ] CDLMATHOLD
- [ ] CDLMORNINGDOJISTAR
- [ ] CDLMORNINGSTAR
- [ ] CDLONNECK
- [ ] CDLPIERCING
- [ ] CDLRICKSHAWMAN
- [ ] CDLRISEFALL3METHODS
- [ ] CDLSEPARATINGLINES
- [ ] CDLSHOOTINGSTAR
- [ ] CDLSHORTLINE
- [ ] CDLSPINNINGTOP
- [ ] CDLSTALLEDPATTERN
- [ ] CDLSTICKSANDWICH
- [ ] CDLTAKURI
- [ ] CDLTASUKIGAP
- [ ] CDLTHRUSTING
- [ ] CDLTRISTAR
- [ ] CDLUNIQUE3RIVER
- [ ] CDLUPSIDEGAP2CROWS
- [ ] CDLXSIDEGAP3METHODS
