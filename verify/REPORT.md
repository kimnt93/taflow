# taflow correctness verification

Date: 2026-08-08 | bars: 10,000 | warm-up split: 9,000 extend + 1,000 append | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.5.1, TA-Lib 0.7.1, taflow 0.1.2

Summary: MATCH: 287

taflow is driven through its canonical classes (mapped from the
TA-Lib name via the /CHECK.md master table). *Batch vs oracle*:
cold `extend` over the full series against the reference;
*continue vs batch*: 9k `extend` + 1k `append` stitched output
bitwise-identical to one-shot batch (chunk invariance); *continue
vs oracle*: the stitched output against the reference. Repeated
native `extend` chunks [1, 10, 1000] are also checked bitwise.

| Function | taflow class | Oracle | Verdict | Batch vs oracle | Continue vs batch | Extend chunks | Continue vs oracle |
|---|---|---|---|---|---|---|---|
| ACCBANDS | AccelerationBands | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ACOS | MathAcos | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AD | AccumulationDistribution | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ADD | MathAdd | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ADOSC | AccumulationDistributionOscillator | TA-Lib | MATCH | pass (err 1.5e-08, nan 0) | yes | yes | pass (err 1.5e-08, nan 0) |
| ADX | AverageDirectionalIndex | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| ADXR | AverageDirectionalIndexRating | TA-Lib | MATCH | pass (err 2.1e-14, nan 0) | yes | yes | pass (err 2.1e-14, nan 0) |
| APO | AbsolutePriceOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AROON | Aroon | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AROONOSC | AroonOscillator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| ASIN | MathAsin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ATAN | MathAtan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ATR | AverageTrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AVGDEV | RollingAverageDeviation | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AVGPRICE | AveragePrice | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | yes | pass (err 5.7e-14, nan 0) |
| BBANDS | BollingerBands | TA-Lib | MATCH | pass (err 7.9e-10, nan 0) | yes | yes | pass (err 7.9e-10, nan 0) |
| BETA | RollingBeta | TA-Lib | MATCH | pass (err 3.9e-12, nan 0) | yes | yes | pass (err 3.9e-12, nan 0) |
| BOP | BalanceOfPower | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CCI | CommodityChannelIndex | TA-Lib | MATCH | pass (err 2.6e-10, nan 0) | yes | yes | pass (err 2.6e-10, nan 0) |
| CDL2CROWS | CandleTwoCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3BLACKCROWS | CandleThreeBlackCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3INSIDE | CandleThreeInside | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3LINESTRIKE | CandleThreeLineStrike | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3OUTSIDE | CandleThreeOutside | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3STARSINSOUTH | CandleThreeStarsInSouth | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3WHITESOLDIERS | CandleThreeWhiteSoldiers | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLABANDONEDBABY | CandleAbandonedBaby | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLADVANCEBLOCK | CandleAdvanceBlock | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLBELTHOLD | CandleBeltHold | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLBREAKAWAY | CandleBreakaway | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCLOSINGMARUBOZU | CandleClosingMarubozu | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCONCEALBABYSWALL | CandleConcealBabySwall | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCOUNTERATTACK | CandleCounterAttack | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDARKCLOUDCOVER | CandleDarkCloudCover | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDOJI | CandleDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDOJISTAR | CandleDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDRAGONFLYDOJI | CandleDragonflyDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLENGULFING | CandleEngulfing | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLEVENINGDOJISTAR | CandleEveningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLEVENINGSTAR | CandleEveningStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLGAPSIDESIDEWHITE | CandleGapSideSideWhite | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLGRAVESTONEDOJI | CandleGravestoneDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHAMMER | CandleHammer | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHANGINGMAN | CandleHangingMan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHARAMI | CandleHarami | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHARAMICROSS | CandleHaramiCross | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIGHWAVE | CandleHighWave | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIKKAKE | CandleHikkake | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIKKAKEMOD | CandleHikkakeModified | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHOMINGPIGEON | CandleHomingPigeon | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLIDENTICAL3CROWS | CandleIdenticalThreeCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLINNECK | CandleInNeck | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLINVERTEDHAMMER | CandleInvertedHammer | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLKICKING | CandleKicking | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLKICKINGBYLENGTH | CandleKickingByLength | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLADDERBOTTOM | CandleLadderBottom | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLONGLEGGEDDOJI | CandleLongLeggedDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLONGLINE | CandleLongLine | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMARUBOZU | CandleMarubozu | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMATCHINGLOW | CandleMatchingLow | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMATHOLD | CandleMatHold | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMORNINGDOJISTAR | CandleMorningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMORNINGSTAR | CandleMorningStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLONNECK | CandleOnNeck | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLPIERCING | CandlePiercing | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLRICKSHAWMAN | CandleRickshawman | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLRISEFALL3METHODS | CandleRiseFallThreeMethods | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSEPARATINGLINES | CandleSeparatingLines | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSHOOTINGSTAR | CandleShootingStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSHORTLINE | CandleShortLine | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSPINNINGTOP | CandleSpinningTop | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSTALLEDPATTERN | CandleStalledPattern | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSTICKSANDWICH | CandleStickSandwich | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTAKURI | CandleTakuri | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTASUKIGAP | CandleTasukiGap | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTHRUSTING | CandleThrusting | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTRISTAR | CandleTriStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLUNIQUE3RIVER | CandleUniqueThreeRiver | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLUPSIDEGAP2CROWS | CandleUpsideGapTwoCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLXSIDEGAP3METHODS | CandleUpDownSideGapThreeMethods | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CEIL | MathCeil | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CMO | ChandeMomentumOscillator | TA-Lib | MATCH | pass (err 6.8e-14, nan 0) | yes | yes | pass (err 6.8e-14, nan 0) |
| CORREL | RollingCorrelation | TA-Lib | MATCH | pass (err 3.1e-11, nan 0) | yes | yes | pass (err 3.1e-11, nan 0) |
| COS | MathCos | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| COSH | MathCosh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DEMA | DoubleExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DIV | MathDivide | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DX | DirectionalMovementIndex | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| EMA | ExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| EXP | MathExp | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| FLOOR | MathFloor | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_DCPERIOD | HilbertTransformDominantCyclePeriod | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_DCPHASE | HilbertTransformDominantCyclePhase | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_PHASOR | HilbertTransformPhasor | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_SINE | HilbertTransformSineWave | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_TRENDLINE | HilbertTransformTrendline | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_TRENDMODE | HilbertTransformTrendMode | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| IMI | IntradayMomentumIndex | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| KAMA | KaufmanAdaptiveMovingAverage | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| LINEARREG | RollingLinearRegression | TA-Lib | MATCH | pass (err 6.0e-13, nan 0) | yes | yes | pass (err 6.0e-13, nan 0) |
| LINEARREG_ANGLE | RollingLinearRegressionAngle | TA-Lib | MATCH | pass (err 4.9e-12, nan 0) | yes | yes | pass (err 4.9e-12, nan 0) |
| LINEARREG_INTERCEPT | RollingLinearRegressionIntercept | TA-Lib | MATCH | pass (err 6.0e-13, nan 0) | yes | yes | pass (err 6.0e-13, nan 0) |
| LINEARREG_SLOPE | RollingLinearRegressionSlope | TA-Lib | MATCH | pass (err 9.1e-14, nan 0) | yes | yes | pass (err 9.1e-14, nan 0) |
| LN | MathLn | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| LOG10 | MathLog10 | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MA | MovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MACD | MovingAverageConvergenceDivergence | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| MACDEXT | MovingAverageConvergenceDivergenceExtended | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MACDFIX | MovingAverageConvergenceDivergenceFixed | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | yes | pass (err 5.7e-14, nan 0) |
| MAMA | MesaAdaptiveMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAVP | VariablePeriodMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAX | RollingMax | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAXINDEX | RollingArgmax | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MEDPRICE | MedianPrice | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MFI | MoneyFlowIndex | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| MIDPOINT | RollingMidpoint | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MIDPRICE | RollingMidprice | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MIN | RollingMin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MININDEX | RollingArgmin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINMAX | RollingMinMax | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINMAXINDEX | RollingMinMaxIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINUS_DI | MinusDirectionalIndicator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| MINUS_DM | MinusDirectionalMovement | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MOM | Momentum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MULT | MathMultiply | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| NATR | NormalizedAverageTrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| OBV | OnBalanceVolume | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| PLUS_DI | PlusDirectionalIndicator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| PLUS_DM | PlusDirectionalMovement | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| PPO | PercentagePriceOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROC | RateOfChange | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| ROCP | RateOfChangePercent | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROCR | RateOfChangeRatio | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROCR100 | RateOfChangeRatioPercent | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| RSI | RelativeStrengthIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SAR | ParabolicSar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SAREXT | ParabolicSarExtended | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SIN | MathSin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SINH | MathSinh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SMA | SimpleMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SQRT | MathSqrt | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STDDEV | RollingStandardDeviation | TA-Lib | MATCH | pass (err 4.0e-10, nan 0) | yes | yes | pass (err 4.0e-10, nan 0) |
| STOCH | StochasticOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STOCHF | FastStochasticOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STOCHRSI | StochasticRelativeStrengthIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SUB | MathSubtract | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SUM | RollingSum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| T3 | TripleExponentialAverage | TA-Lib | MATCH | pass (err 5.7e-13, nan 0) | yes | yes | pass (err 5.7e-13, nan 0) |
| TAN | MathTan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TANH | MathTanh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TEMA | TripleExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TRANGE | TrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TRIMA | TriangularMovingAverage | TA-Lib | MATCH | pass (err 6.8e-13, nan 0) | yes | yes | pass (err 6.8e-13, nan 0) |
| TRIX | TripleExponentialRateOfChange | TA-Lib | MATCH | pass (err 1.1e-14, nan 0) | yes | yes | pass (err 1.1e-14, nan 0) |
| TSF | RollingTimeSeriesForecast | TA-Lib | MATCH | pass (err 6.8e-13, nan 0) | yes | yes | pass (err 6.8e-13, nan 0) |
| TYPPRICE | TypicalPrice | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | yes | pass (err 5.7e-14, nan 0) |
| ULTOSC | UltimateOscillator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| VAR | RollingVariance | TA-Lib | MATCH | pass (err 3.1e-10, nan 0) | yes | yes | pass (err 3.1e-10, nan 0) |
| WCLPRICE | WeightedClose | TA-Lib | MATCH | pass (err 5.7e-14, nan 0) | yes | yes | pass (err 5.7e-14, nan 0) |
| WILLR | WilliamsPercentR | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| WMA | WeightedMovingAverage | TA-Lib | MATCH | pass (err 1.9e-10, nan 0) | yes | yes | pass (err 1.9e-10, nan 0) |
| amihud | Amihud | self | MATCH | — | yes | yes | — |
| anchored_vwap | AnchoredVolumeWeightedAveragePrice | self | MATCH | — | yes | yes | — |
| arnaud_legoux_moving_average | ArnaudLegouxMovingAverage | self | MATCH | — | yes | yes | — |
| average_daily_dollar_value | AverageDailyDollarValue | self | MATCH | — | yes | yes | — |
| awesome_oscillator | AwesomeOscillator | self | MATCH | — | yes | yes | — |
| bars_since | BarsSince | self | MATCH | — | yes | yes | — |
| break_of_structure_change_of_character | BreakOfStructureChangeOfCharacter | self | MATCH | — | yes | yes | — |
| chaikin_money_flow | ChaikinMoneyFlow | self | MATCH | — | yes | yes | — |
| chaikin_volatility | ChaikinVolatility | self | MATCH | — | yes | yes | — |
| close_to_close_sigma | CloseToCloseSigma | self | MATCH | — | yes | yes | — |
| crossover | Crossover | self | MATCH | — | yes | yes | — |
| crossunder | Crossunder | self | MATCH | — | yes | yes | — |
| cumulative_count | CumulativeCount | self | MATCH | — | yes | yes | — |
| cumulative_maximum | CumulativeMaximum | self | MATCH | — | yes | yes | — |
| cumulative_minimum | CumulativeMinimum | self | MATCH | — | yes | yes | — |
| cumulative_product | CumulativeProduct | self | MATCH | — | yes | yes | — |
| cumulative_sum | CumulativeSum | self | MATCH | — | yes | yes | — |
| cumulative_sum_control_chart | CumulativeSumControlChart | self | MATCH | — | yes | yes | — |
| decay_linear | DecayLinear | self | MATCH | — | yes | yes | — |
| detrended_price_oscillator | DetrendedPriceOscillator | self | MATCH | — | yes | yes | — |
| donchian_channels | Donchian | self | MATCH | — | yes | yes | — |
| drawdown | Drawdown | self | MATCH | — | yes | yes | — |
| ease_of_movement | EaseOfMovement | self | MATCH | — | yes | yes | — |
| equal_highs_lows | EqualHighsLows | self | MATCH | — | yes | yes | — |
| even_better_sinewave | EvenBetterSinewave | self | MATCH | — | yes | yes | — |
| ewm_corr | ExponentiallyWeightedCorrelation | self | MATCH | — | yes | yes | — |
| ewm_cov | ExponentiallyWeightedCovariance | self | MATCH | — | yes | yes | — |
| ewm_std | ExponentiallyWeightedStandardDeviation | pandas | MATCH | pass (err 7.8e-14, nan 0) | yes | yes | pass (err 7.8e-14, nan 0) |
| ewm_var | ExponentiallyWeightedVariance | pandas | MATCH | pass (err 1.9e-12, nan 0) | yes | yes | pass (err 1.9e-12, nan 0) |
| exponentially_weighted_sum | ExponentiallyWeightedSum | self | MATCH | — | yes | yes | — |
| fair_value_gap | FairValueGap | self | MATCH | — | yes | yes | — |
| falling | Falling | self | MATCH | — | yes | yes | — |
| fib_retracement | FibonacciRetracement | self | MATCH | — | yes | yes | — |
| fisher_transform | FisherTransform | self | MATCH | — | yes | yes | — |
| force_index | ForceIndex | self | MATCH | — | yes | yes | — |
| frac_diff | FracDiff | self | MATCH | — | yes | yes | — |
| fractal_dimension | FractalDimension | self | MATCH | — | yes | yes | — |
| gap_down | GapDown | self | MATCH | — | yes | yes | — |
| gap_up | GapUp | self | MATCH | — | yes | yes | — |
| garman_klass | GarmanKlass | self | MATCH | — | yes | yes | — |
| garman_klass_yang_zhang | GarmanKlassYangZhang | self | MATCH | — | yes | yes | — |
| hedge_ratio | HedgeRatio | self | MATCH | — | yes | yes | — |
| heikin_ashi | HeikinAshi | self | MATCH | — | yes | yes | — |
| higher_high | HigherHigh | self | MATCH | — | yes | yes | — |
| highest_since | HighestSince | self | MATCH | — | yes | yes | — |
| hull_moving_average | HullMovingAverage | self | MATCH | — | yes | yes | — |
| hurst | Hurst | self | MATCH | — | yes | yes | — |
| ichimoku | Ichimoku | self | MATCH | — | yes | yes | — |
| inside_bar | InsideBar | self | MATCH | — | yes | yes | — |
| jma | JurikMovingAverage | self | MATCH | — | yes | yes | — |
| kalman_hedge_ratio | KalmanHedgeRatio | self | MATCH | — | yes | yes | — |
| keltner_channels | KeltnerChannels | self | MATCH | — | yes | yes | — |
| know_sure_thing | KnowSureThing | self | MATCH | — | yes | yes | — |
| kvo | KlingerVolumeOscillator | self | MATCH | — | yes | yes | — |
| lag | Lag | self | MATCH | — | yes | yes | — |
| laguerre_rsi | LaguerreRelativeStrengthIndex | self | MATCH | — | yes | yes | — |
| liquidity | Liquidity | self | MATCH | — | yes | yes | — |
| log_return | LogReturn | self | MATCH | — | yes | yes | — |
| lower_low | LowerLow | self | MATCH | — | yes | yes | — |
| lowest_since | LowestSince | self | MATCH | — | yes | yes | — |
| mass_index | MassIndex | self | MATCH | — | yes | yes | — |
| math_abs | MathAbs | self | MATCH | — | yes | yes | — |
| math_acosh | MathAcosh | self | MATCH | — | yes | yes | — |
| math_asinh | MathAsinh | self | MATCH | — | yes | yes | — |
| math_atanh | MathAtanh | self | MATCH | — | yes | yes | — |
| math_cbrt | MathCbrt | self | MATCH | — | yes | yes | — |
| math_cot | MathCot | self | MATCH | — | yes | yes | — |
| math_degrees | MathDegrees | self | MATCH | — | yes | yes | — |
| math_log1p | MathLog1p | self | MATCH | — | yes | yes | — |
| math_radians | MathRadians | self | MATCH | — | yes | yes | — |
| mcginley | McGinleyDynamic | self | MATCH | — | yes | yes | — |
| negative_volume_index | NegativeVolumeIndex | self | MATCH | — | yes | yes | — |
| opening_range | OpeningRange | self | MATCH | — | yes | yes | — |
| order_block | OrderBlock | self | MATCH | — | yes | yes | — |
| ornstein_uhlenbeck_half_life | OrnsteinUhlenbeckHalfLife | self | MATCH | — | yes | yes | — |
| outside_bar | OutsideBar | self | MATCH | — | yes | yes | — |
| parkinson | Parkinson | self | MATCH | — | yes | yes | — |
| pivot_points | PivotPoints | self | MATCH | — | yes | yes | — |
| pmax | ParabolicMovingAverageStop | self | MATCH | — | yes | yes | — |
| positive_volume_index | PositiveVolumeIndex | self | MATCH | — | yes | yes | — |
| premium_discount | PremiumDiscount | self | MATCH | — | yes | yes | — |
| previous_high_low | PreviousHighLow | self | MATCH | — | yes | yes | — |
| retracements | Retracements | self | MATCH | — | yes | yes | — |
| rising | Rising | self | MATCH | — | yes | yes | — |
| rmi | RelativeMomentumIndex | self | MATCH | — | yes | yes | — |
| rogers_satchell | RogersSatchell | self | MATCH | — | yes | yes | — |
| roll_spread | RollSpread | self | MATCH | — | yes | yes | — |
| rolling_alpha | RollingAlpha | self | MATCH | — | yes | yes | — |
| rolling_autocorr | RollingAutocorr | self | MATCH | — | yes | yes | — |
| rolling_calmar | RollingCalmar | self | MATCH | — | yes | yes | — |
| rolling_cov | RollingCov | pandas | MATCH | pass (err 1.3e-11, nan 0) | yes | yes | pass (err 1.3e-11, nan 0) |
| rolling_entropy | RollingEntropy | self | MATCH | — | yes | yes | — |
| rolling_information_ratio | RollingInformationRatio | self | MATCH | — | yes | yes | — |
| rolling_kurtosis | RollingKurtosis | pandas | MATCH | pass (err 1.3e-15, nan 0) | yes | yes | pass (err 1.3e-15, nan 0) |
| rolling_median | RollingMedian | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_mode | RollingMode | self | MATCH | — | yes | yes | — |
| rolling_quantile | RollingQuantile | pandas | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_rank | RollingRank | self | MATCH | — | yes | yes | — |
| rolling_sharpe | RollingSharpe | self | MATCH | — | yes | yes | — |
| rolling_skew | RollingSkew | pandas | MATCH | pass (err 8.9e-16, nan 0) | yes | yes | pass (err 8.9e-16, nan 0) |
| rolling_sortino | RollingSortino | self | MATCH | — | yes | yes | — |
| rolling_vwap | RollingVolumeWeightedAveragePrice | self | MATCH | — | yes | yes | — |
| rolling_winsorize | RollingWinsorize | self | MATCH | — | yes | yes | — |
| rolling_zscore | RollingZScore | pandas | MATCH | pass (err 6.4e-10, nan 0) | yes | yes | pass (err 6.4e-10, nan 0) |
| schaff_trend_cycle | SchaffTrendCycle | self | MATCH | — | yes | yes | — |
| session_volume_levels | SessionVolumeLevels | self | MATCH | — | yes | yes | — |
| sessions | Sessions | self | MATCH | — | yes | yes | — |
| signal_delay | SignalDelay | self | MATCH | — | yes | yes | — |
| signed_power | SignedPower | self | MATCH | — | yes | yes | — |
| spread_zscore | SpreadZScore | self | MATCH | — | yes | yes | — |
| squeeze | Squeeze | self | MATCH | — | yes | yes | — |
| squeeze_pro | SqueezePro | self | MATCH | — | yes | yes | — |
| ssl_channel | SmoothedTrendChannel | self | MATCH | — | yes | yes | — |
| supertrend | Supertrend | self | MATCH | — | yes | yes | — |
| swing_highs_lows | SwingHighLow | self | MATCH | — | yes | yes | — |
| td_sequential | TomDeMarkSequential | self | MATCH | — | yes | yes | — |
| time_series_rank | TimeSeriesRank | self | MATCH | — | yes | yes | — |
| true_strength_index | TrueStrengthIndex | self | MATCH | — | yes | yes | — |
| ulcer_index | UlcerIndex | self | MATCH | — | yes | yes | — |
| value_when | ValueWhen | self | MATCH | — | yes | yes | — |
| vidya | VariableIndexDynamicAverage | self | MATCH | — | yes | yes | — |
| volume_price_trend | VolumePriceTrend | self | MATCH | — | yes | yes | — |
| volume_weighted_moving_average | VolumeWeightedMovingAverage | self | MATCH | — | yes | yes | — |
| vortex | Vortex | self | MATCH | — | yes | yes | — |
| yang_zhang | YangZhang | self | MATCH | — | yes | yes | — |
| zero_lag_exponential_moving_average | ZeroLagExponentialMovingAverage | self | MATCH | — | yes | yes | — |

## Follow-ups

- Mismatches: none
- Errors (class/mapping/runtime): none
- Compared at TA-Lib defaults only (unmapped params): STDDEV, VAR, CDLABANDONEDBABY, CDLDARKCLOUDCOVER, CDLEVENINGDOJISTAR, CDLEVENINGSTAR, CDLMATHOLD, CDLMORNINGDOJISTAR, CDLMORNINGSTAR
