# Full TA stateful-extension checklist

This is the authoritative delivery checklist.  A checked item means its
stateful extension exists in Rust and Python, has batch-parity tests, and is
included in the streaming benchmark where applicable.  A `B✓` annotation
records a separately verified batch function.  Do not check a stateful item
merely because the legacy batch function exists.

The completed stateful surface is tracked function-by-function below, including
the price transforms, math transforms, and pointwise arithmetic operators.
`MACDEXT` still needs distinct state semantics and therefore remains open.

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
- [ ] MAVP
- [ ] HT_TRENDLINE
- [x] MA

## Momentum Indicators

- [x] IMI
- [x] RSI
- [x] MACD
- [ ] MACDEXT
- [x] MACDFIX
- [x] STOCH
- [x] STOCHF
- [x] STOCHRSI
- [ ] ADX
- [ ] ADXR
- [ ] CCI
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
- [ ] MFI
- [ ] TRIX
- [ ] ULTOSC
- [ ] DX
- [ ] PLUS_DI
- [ ] MINUS_DI
- [ ] PLUS_DM
- [ ] MINUS_DM

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

- [ ] HT_DCPERIOD
- [ ] HT_DCPHASE
- [ ] HT_PHASOR
- [ ] HT_SINE
- [ ] HT_TRENDMODE

## Pattern Recognition

- [ ] CDLDOJI
- [ ] CDLHAMMER
- [ ] CDLENGULFING
- [ ] CDL2CROWS
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
