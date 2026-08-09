# Priority-selected correctness sources

One oracle is selected per indicator using: **TA-Lib > NumPy > Polars > pandas > pandas-ta-classic > pinned GitHub**. `VARIANT` is a documented semantic difference, not a failed comparison; `INVARIANT` rows have no external oracle.

Matches: **327** | Documented variants: **14** | Self-invariant outputs: **80** | Failures: **0**

| TAFlow class ↔ oracle API | Output | Selected source | Version | Verdict | Max error | NaN | Note |
|---|---|---|---|---:|---:|---:|---|
| `AbsolutePriceOscillator` ↔ `APO` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `AccelerationBands` ↔ `ACCBANDS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `AccumulationDistribution` ↔ `AD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `AccumulationDistributionOscillator` ↔ `ADOSC` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.490e-08` | 0 | external parity plus bitwise lifecycle invariance |
| `Amihud` ↔ `self.amihud` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `AnchoredVolumeWeightedAveragePrice` ↔ `pandas.core.groupby.SeriesGroupBy.cumsum` | `all` | [self](https://pandas.pydata.org/docs/reference/api/pandas.core.groupby.SeriesGroupBy.cumsum.html) | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ArnaudLegouxMovingAverage` ↔ `pandas-ta-classic.arnaud_legoux_moving_average` | `alma` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `5.611e+00` | 0 | independently compared; documented initialization/formula convention differs |
| `Aroon` ↔ `AROON` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `AroonOscillator` ↔ `AROONOSC` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `AverageDailyDollarValue` ↔ `self.average_daily_dollar_value` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `AverageDirectionalIndex` ↔ `ADX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `AverageDirectionalIndexRating` ↔ `ADXR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.132e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `AveragePrice` ↔ `AVGPRICE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `AverageTrueRange` ↔ `ATR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `AwesomeOscillator` ↔ `pandas_ta_classic.ao` | `ao` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `8.527e-14` | 0 |  |
| `BalanceOfPower` ↔ `BOP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `BarsSince` ↔ `self.bars_since` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `BollingerBands` ↔ `BBANDS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `7.927e-10` | 0 | external parity plus bitwise lifecycle invariance |
| `BreakOfStructureChangeOfCharacter` ↔ `self.break_of_structure_change_of_character` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CandleAbandonedBaby` ↔ `CDLABANDONEDBABY` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleAdvanceBlock` ↔ `CDLADVANCEBLOCK` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleBeltHold` ↔ `CDLBELTHOLD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleBreakaway` ↔ `CDLBREAKAWAY` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleClosingMarubozu` ↔ `CDLCLOSINGMARUBOZU` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleConcealBabySwall` ↔ `CDLCONCEALBABYSWALL` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleCounterAttack` ↔ `CDLCOUNTERATTACK` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleDarkCloudCover` ↔ `CDLDARKCLOUDCOVER` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleDoji` ↔ `CDLDOJI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleDojiStar` ↔ `CDLDOJISTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleDragonflyDoji` ↔ `CDLDRAGONFLYDOJI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleEngulfing` ↔ `CDLENGULFING` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleEveningDojiStar` ↔ `CDLEVENINGDOJISTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleEveningStar` ↔ `CDLEVENINGSTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleGapSideSideWhite` ↔ `CDLGAPSIDESIDEWHITE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleGravestoneDoji` ↔ `CDLGRAVESTONEDOJI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHammer` ↔ `CDLHAMMER` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHangingMan` ↔ `CDLHANGINGMAN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHarami` ↔ `CDLHARAMI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHaramiCross` ↔ `CDLHARAMICROSS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHighWave` ↔ `CDLHIGHWAVE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHikkake` ↔ `CDLHIKKAKE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHikkakeModified` ↔ `CDLHIKKAKEMOD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleHomingPigeon` ↔ `CDLHOMINGPIGEON` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleIdenticalThreeCrows` ↔ `CDLIDENTICAL3CROWS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleInNeck` ↔ `CDLINNECK` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleInvertedHammer` ↔ `CDLINVERTEDHAMMER` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleKicking` ↔ `CDLKICKING` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleKickingByLength` ↔ `CDLKICKINGBYLENGTH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleLadderBottom` ↔ `CDLLADDERBOTTOM` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleLongLeggedDoji` ↔ `CDLLONGLEGGEDDOJI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleLongLine` ↔ `CDLLONGLINE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleMarubozu` ↔ `CDLMARUBOZU` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleMatHold` ↔ `CDLMATHOLD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleMatchingLow` ↔ `CDLMATCHINGLOW` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleMorningDojiStar` ↔ `CDLMORNINGDOJISTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleMorningStar` ↔ `CDLMORNINGSTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleOnNeck` ↔ `CDLONNECK` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandlePiercing` ↔ `CDLPIERCING` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleRickshawman` ↔ `CDLRICKSHAWMAN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleRiseFallThreeMethods` ↔ `CDLRISEFALL3METHODS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleSeparatingLines` ↔ `CDLSEPARATINGLINES` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleShootingStar` ↔ `CDLSHOOTINGSTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleShortLine` ↔ `CDLSHORTLINE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleSpinningTop` ↔ `CDLSPINNINGTOP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleStalledPattern` ↔ `CDLSTALLEDPATTERN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleStickSandwich` ↔ `CDLSTICKSANDWICH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleTakuri` ↔ `CDLTAKURI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleTasukiGap` ↔ `CDLTASUKIGAP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeBlackCrows` ↔ `CDL3BLACKCROWS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeInside` ↔ `CDL3INSIDE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeLineStrike` ↔ `CDL3LINESTRIKE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeOutside` ↔ `CDL3OUTSIDE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeStarsInSouth` ↔ `CDL3STARSINSOUTH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThreeWhiteSoldiers` ↔ `CDL3WHITESOLDIERS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleThrusting` ↔ `CDLTHRUSTING` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleTriStar` ↔ `CDLTRISTAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleTwoCrows` ↔ `CDL2CROWS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleUniqueThreeRiver` ↔ `CDLUNIQUE3RIVER` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleUpDownSideGapThreeMethods` ↔ `CDLXSIDEGAP3METHODS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `CandleUpsideGapTwoCrows` ↔ `CDLUPSIDEGAP2CROWS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `ChaikinMoneyFlow` ↔ `pandas_ta_classic.cmf` | `cmf` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `3.993e-14` | 0 |  |
| `ChaikinVolatility` ↔ `pandas-ta-classic.chaikin_volatility` | `chaikin_volatility` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `2.563e+00` | 9 | independently compared; documented initialization/formula convention differs |
| `ChandeMomentumOscillator` ↔ `CMO` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `6.817e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `CloseToCloseSigma` ↔ `self.close_to_close_sigma` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CommodityChannelIndex` ↔ `CCI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.387e-11` | 0 | external parity plus bitwise lifecycle invariance |
| `Crossover` ↔ `pandas-ta-classic.crossover` | `crossover` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `Crossunder` ↔ `pandas-ta-classic.crossunder` | `crossunder` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `CumulativeCount` ↔ `self.cumulative_count` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CumulativeMaximum` ↔ `self.cumulative_maximum` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CumulativeMinimum` ↔ `self.cumulative_minimum` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CumulativeProduct` ↔ `self.cumulative_product` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CumulativeSum` ↔ `self.cumulative_sum` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `CumulativeSumControlChart` ↔ `self.cumulative_sum_control_chart` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `DecayLinear` ↔ `self.decay_linear` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `DetrendedPriceOscillator` ↔ `pandas_ta_classic.dpo` | `dpo` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.137e-13` | 0 |  |
| `DirectionalMovementIndex` ↔ `DX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `DonchianChannels` ↔ `pandas_ta_classic.donchian` | `lower` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `DonchianChannels` ↔ `pandas_ta_classic.donchian` | `mid` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `DonchianChannels` ↔ `pandas_ta_classic.donchian` | `upper` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `DoubleExponentialMovingAverage` ↔ `DEMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `Drawdown` ↔ `self.drawdown` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `EaseOfMovement` ↔ `pandas-ta-classic.ease_of_movement` | `ease_of_movement` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `4.531e+03` | 0 | independently compared; documented initialization/formula convention differs |
| `EqualHighsLows` ↔ `self.equal_highs_lows` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `EvenBetterSinewave` ↔ `pandas_ta_classic.ebsw` | `ebsw` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `EvenBetterSinewave` ↔ `pandas_ta_classic.ebsw` | `ebsw[length=60]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `ExponentialMovingAverage` ↔ `EMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `ExponentiallyWeightedCorrelation` ↔ `self.ewm_corr` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ExponentiallyWeightedCovariance` ↔ `self.ewm_cov` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ExponentiallyWeightedStandardDeviation` ↔ `pandas.ewm_std` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `7.816e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `ExponentiallyWeightedSum` ↔ `self.exponentially_weighted_sum` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ExponentiallyWeightedVariance` ↔ `pandas.ewm_var` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `1.904e-12` | 0 | external parity plus bitwise lifecycle invariance |
| `FairValueGap` ↔ `self.fair_value_gap` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `Falling` ↔ `self.falling` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `FastStochasticOscillator` ↔ `STOCHF` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `FibonacciRetracement` ↔ `self.fib_retracement` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `FisherTransform` ↔ `pandas_ta_classic.fisher` | `fisher` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `FisherTransform` ↔ `pandas_ta_classic.fisher` | `fisher[length=11]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `FisherTransform` ↔ `pandas_ta_classic.fisher` | `fisher[length=21]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `ForceIndex` ↔ `pandas_ta_classic.efi` | `force_index` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | taflow exposes the unsmoothed one-bar force; pandas-ta EFI length=1 |
| `FracDiff` ↔ `self.frac_diff` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `FractalDimension` ↔ `self.fractal_dimension` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `GapDown` ↔ `self.gap_down` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `GapUp` ↔ `self.gap_up` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `GarmanKlass` ↔ `self.garman_klass` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `GarmanKlassYangZhang` ↔ `self.garman_klass_yang_zhang` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `HedgeRatio` ↔ `self.hedge_ratio` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `HeikinAshi` ↔ `pandas-ta-classic.heikin_ashi` | `close` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `HeikinAshi` ↔ `pandas-ta-classic.heikin_ashi` | `high` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `HeikinAshi` ↔ `pandas-ta-classic.heikin_ashi` | `low` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `HeikinAshi` ↔ `pandas-ta-classic.heikin_ashi` | `open` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `HigherHigh` ↔ `self.higher_high` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `HighestSince` ↔ `self.highest_since` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `HilbertTransformDominantCyclePeriod` ↔ `HT_DCPERIOD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HilbertTransformDominantCyclePhase` ↔ `HT_DCPHASE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HilbertTransformPhasor` ↔ `HT_PHASOR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HilbertTransformSineWave` ↔ `HT_SINE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HilbertTransformTrendMode` ↔ `HT_TRENDMODE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HilbertTransformTrendline` ↔ `HT_TRENDLINE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `HullMovingAverage` ↔ `pandas_ta_classic.hma` | `hma` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `Hurst` ↔ `self.hurst` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `Ichimoku` ↔ `pandas-ta-classic.ichimoku` | `chikou` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `Ichimoku` ↔ `pandas-ta-classic.ichimoku` | `kijun` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `Ichimoku` ↔ `pandas-ta-classic.ichimoku` | `span_a` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `Ichimoku` ↔ `pandas-ta-classic.ichimoku` | `span_b` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `Ichimoku` ↔ `pandas-ta-classic.ichimoku` | `tenkan` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `InsideBar` ↔ `self.inside_bar` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `IntradayMomentumIndex` ↔ `IMI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `JurikMovingAverage` ↔ `pandas-ta-classic.jma` | `jma` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `2.842e-14` | 0 |  |
| `JurikMovingAverage` ↔ `pandas-ta-classic.jma` | `jma[length=1,phase=0]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `JurikMovingAverage` ↔ `pandas-ta-classic.jma` | `jma[length=2,phase=-100]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 |  |
| `JurikMovingAverage` ↔ `pandas-ta-classic.jma` | `jma[length=21,phase=35]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `2.842e-14` | 0 |  |
| `JurikMovingAverage` ↔ `pandas-ta-classic.jma` | `jma[length=7,phase=100]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `4.263e-14` | 0 |  |
| `KalmanHedgeRatio` ↔ `self.kalman_hedge_ratio` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `KaufmanAdaptiveMovingAverage` ↔ `KAMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `KeltnerChannels` ↔ `pandas-ta-classic.keltner_channels` | `lower` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `3.751e+00` | 20 | independently compared; documented initialization/formula convention differs |
| `KeltnerChannels` ↔ `pandas-ta-classic.keltner_channels` | `middle` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `4.587e-01` | 19 | independently compared; documented initialization/formula convention differs |
| `KeltnerChannels` ↔ `pandas-ta-classic.keltner_channels` | `upper` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `3.846e+00` | 20 | independently compared; documented initialization/formula convention differs |
| `KlingerVolumeOscillator` ↔ `pandas-ta-classic.kvo` | `kvo` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `2.183e-10` | 0 |  |
| `KlingerVolumeOscillator` ↔ `pandas-ta-classic.kvo` | `kvo[fast=5,slow=8,signal=3]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `2.910e-10` | 0 |  |
| `KlingerVolumeOscillator` ↔ `pandas-ta-classic.kvo` | `signal` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.914e-10` | 0 |  |
| `KlingerVolumeOscillator` ↔ `pandas-ta-classic.kvo` | `signal[fast=5,slow=8,signal=3]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `2.146e-10` | 0 |  |
| `KnowSureThing` ↔ `pandas-ta-classic.know_sure_thing` | `kst` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `1.824e+04` | 0 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| `KnowSureThing` ↔ `pandas-ta-classic.know_sure_thing` | `signal` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `1.782e+04` | 8 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| `Lag` ↔ `self.lag` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[constant]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[gamma=0.1]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[gamma=0.25]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[gamma=0.9]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[minimum]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[monotonic]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `LaguerreRelativeStrengthIndex` ↔ `pandas_ta_classic.lrsi` | `lrsi[repeated]` | [pandas-ta-classic](https://github.com/xgboosted/pandas-ta-classic/blob/0.6.52/pandas_ta_classic/momentum/lrsi.py) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `Liquidity` ↔ `self.liquidity` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `LogReturn` ↔ `pandas_ta_classic.log_return` | `log_return` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `LowerLow` ↔ `self.lower_low` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `LowestSince` ↔ `self.lowest_since` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MassIndex` ↔ `pandas-ta-classic.mass_index` | `mass` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `4.737e-02` | 0 | taflow follows bukosabino/ta EMA initialization |
| `MathAbs` ↔ `self.math_abs` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathAcos` ↔ `ACOS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathAcosh` ↔ `self.math_acosh` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathAdd` ↔ `ADD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathAsin` ↔ `ASIN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathAsinh` ↔ `self.math_asinh` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathAtan` ↔ `ATAN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathAtanh` ↔ `self.math_atanh` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathCbrt` ↔ `self.math_cbrt` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathCeil` ↔ `CEIL` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathCos` ↔ `COS` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathCosh` ↔ `COSH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathCot` ↔ `self.math_cot` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathDegrees` ↔ `self.math_degrees` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathDivide` ↔ `DIV` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathExp` ↔ `EXP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathFloor` ↔ `FLOOR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathLn` ↔ `LN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathLog10` ↔ `LOG10` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathLog1p` ↔ `self.math_log1p` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathMultiply` ↔ `MULT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathRadians` ↔ `self.math_radians` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `MathSin` ↔ `SIN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathSinh` ↔ `SINH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathSqrt` ↔ `SQRT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathSubtract` ↔ `SUB` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathTan` ↔ `TAN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MathTanh` ↔ `TANH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `McGinleyDynamic` ↔ `pandas-ta-classic.mcginley` | `mcginley` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `MedianPrice` ↔ `MEDPRICE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MesaAdaptiveMovingAverage` ↔ `MAMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MinusDirectionalIndicator` ↔ `MINUS_DI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `MinusDirectionalMovement` ↔ `MINUS_DM` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `Momentum` ↔ `MOM` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MoneyFlowIndex` ↔ `MFI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `MovingAverage` ↔ `MA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MovingAverageConvergenceDivergence` ↔ `MACD` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `MovingAverageConvergenceDivergenceExtended` ↔ `MACDEXT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `MovingAverageConvergenceDivergenceFixed` ↔ `MACDFIX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `NegativeVolumeIndex` ↔ `pandas-ta-classic.negative_volume_index` | `nvi` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `5.278e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| `NormalizedAverageTrueRange` ↔ `NATR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `OnBalanceVolume` ↔ `OBV` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `OpeningRange` ↔ `self.opening_range` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `OrderBlock` ↔ `self.order_block` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `OrnsteinUhlenbeckHalfLife` ↔ `self.ornstein_uhlenbeck_half_life` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `OutsideBar` ↔ `self.outside_bar` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ParabolicMovingAverageStop` ↔ `pandas-ta-classic.pmax` | `stop` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `5.684e-14` | 0 |  |
| `ParabolicSar` ↔ `SAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `ParabolicSarExtended` ↔ `SAREXT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `Parkinson` ↔ `self.parkinson` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `PercentagePriceOscillator` ↔ `PPO` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `PivotPoints` ↔ `self.pivot_points` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `PlusDirectionalIndicator` ↔ `PLUS_DI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `PlusDirectionalMovement` ↔ `PLUS_DM` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `PositiveVolumeIndex` ↔ `pandas-ta-classic.positive_volume_index` | `pvi` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `4.481e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| `PremiumDiscount` ↔ `self.premium_discount` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `PreviousHighLow` ↔ `self.previous_high_low` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RateOfChange` ↔ `ROC` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `RateOfChangePercent` ↔ `ROCP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RateOfChangeRatio` ↔ `ROCR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RateOfChangeRatioPercent` ↔ `ROCR100` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RelativeMomentumIndex` ↔ `wickra.RMI` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RelativeStrengthIndex` ↔ `RSI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `Retracements` ↔ `self.retracements` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `Rising` ↔ `self.rising` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RogersSatchell` ↔ `self.rogers_satchell` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollSpread` ↔ `self.roll_spread` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingAlpha` ↔ `self.rolling_alpha` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingArgmax` ↔ `MAXINDEX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingArgmin` ↔ `MININDEX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingAutocorr` ↔ `self.rolling_autocorr` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingAverageDeviation` ↔ `AVGDEV` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingBeta` ↔ `BETA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingCalmar` ↔ `self.rolling_calmar` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingCorrelation` ↔ `CORREL` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingCov` ↔ `pandas.rolling_cov` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `1.282e-11` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingEntropy` ↔ `self.rolling_entropy` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingInformationRatio` ↔ `self.rolling_information_ratio` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingKurtosis` ↔ `pandas.rolling_kurtosis` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `1.332e-15` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingLinearRegression` ↔ `LINEARREG` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.969e-13` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingLinearRegressionAngle` ↔ `LINEARREG_ANGLE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `4.924e-12` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingLinearRegressionIntercept` ↔ `LINEARREG_INTERCEPT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.969e-13` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingLinearRegressionSlope` ↔ `LINEARREG_SLOPE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `9.137e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMax` ↔ `MAX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMedian` ↔ `pandas.rolling_median` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMidpoint` ↔ `MIDPOINT` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMidprice` ↔ `MIDPRICE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMin` ↔ `MIN` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMinMax` ↔ `MINMAX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMinMaxIndex` ↔ `MINMAXINDEX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingMode` ↔ `self.rolling_mode` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingQuantile` ↔ `pandas.rolling_quantile` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingRank` ↔ `self.rolling_rank` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingSharpe` ↔ `self.rolling_sharpe` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingSkew` ↔ `pandas.rolling_skew` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `8.882e-16` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingSortino` ↔ `self.rolling_sortino` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingStandardDeviation` ↔ `STDDEV` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingSum` ↔ `SUM` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingTimeSeriesForecast` ↔ `TSF` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `6.821e-13` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingVariance` ↔ `VAR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `RollingVolumeWeightedAveragePrice` ↔ `self.rolling_vwap` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingWinsorize` ↔ `self.rolling_winsorize` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `RollingZScore` ↔ `pandas.rolling_zscore` | `all` | [pandas](https://pandas.pydata.org/docs/reference/window.html) | `3.0.5` | MATCH | `4.771e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `SchaffTrendCycle` ↔ `pandas_ta_classic.stc` | `macd` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `7.105e-14` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `SchaffTrendCycle` ↔ `pandas_ta_classic.stc` | `stc` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.000e-08` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `SchaffTrendCycle` ↔ `pandas_ta_classic.stc` | `stochastic` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `SessionVolumeLevels` ↔ `self.session_volume_levels` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `Sessions` ↔ `self.sessions` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `SignalDelay` ↔ `self.signal_delay` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `SignedPower` ↔ `self.signed_power` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `SimpleMovingAverage` ↔ `SMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `SmoothedTrendChannel` ↔ `self.ssl_channel` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `SpreadZScore` ↔ `self.spread_zscore` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `Squeeze` ↔ `pandas-ta-classic.squeeze` | `momentum` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `3.553e-15` | 0 |  |
| `Squeeze` ↔ `pandas-ta-classic.squeeze` | `no` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `Squeeze` ↔ `pandas-ta-classic.squeeze` | `off` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `Squeeze` ↔ `pandas-ta-classic.squeeze` | `on` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `momentum` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `3.553e-15` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `no` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `off` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `on_narrow` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `on_normal` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `SqueezePro` ↔ `pandas-ta-classic.squeeze_pro` | `on_wide` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `StochasticOscillator` ↔ `STOCH` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `StochasticRelativeStrengthIndex` ↔ `STOCHRSI` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `Supertrend` ↔ `pandas-ta-classic.supertrend` | `direction` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `Supertrend` ↔ `pandas-ta-classic.supertrend` | `long` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `Supertrend` ↔ `pandas-ta-classic.supertrend` | `short` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `Supertrend` ↔ `pandas-ta-classic.supertrend` | `trend` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `SwingHighsLows` ↔ `self.swing_highs_lows` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `TimeSeriesRank` ↔ `self.time_series_rank` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `TomDeMarkSequential` ↔ `pandas-ta-classic.td_sequential` | `buy` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| `TomDeMarkSequential` ↔ `pandas-ta-classic.td_sequential` | `sell` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| `TriangularMovingAverage` ↔ `TRIMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `6.821e-13` | 0 | external parity plus bitwise lifecycle invariance |
| `TripleExponentialAverage` ↔ `T3` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.684e-13` | 0 | external parity plus bitwise lifecycle invariance |
| `TripleExponentialMovingAverage` ↔ `TEMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `TripleExponentialRateOfChange` ↔ `TRIX` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.110e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `TrueRange` ↔ `TRANGE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `TrueStrengthIndex` ↔ `pandas-ta-classic.true_strength_index` | `tsi` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `7.286e+01` | 36 | independently compared; documented initialization/formula convention differs |
| `TypicalPrice` ↔ `TYPPRICE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `UlcerIndex` ↔ `pandas-ta-classic.ulcer_index` | `ulcer_index` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `1.169e+01` | 13 | independently compared; documented initialization/formula convention differs |
| `UltimateOscillator` ↔ `ULTOSC` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `ValueWhen` ↔ `self.value_when` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[constant]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[length=1]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[length=2]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[length=30]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 |  |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[minimum]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[monotonic]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `VariableIndexDynamicAverage` ↔ `pandas-ta-classic.vidya` | `vidya[repeated]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| `VariablePeriodMovingAverage` ↔ `MAVP` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle invariance |
| `VolumePriceTrend` ↔ `pandas-ta-classic.volume_price_trend` | `volume_price_trend` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | VARIANT | `6.091e+07` | 0 | independently compared; documented initialization/formula convention differs |
| `VolumeWeightedMovingAverage` ↔ `pandas_ta_classic.vwma` | `vwma` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `5.684e-14` | 0 |  |
| `Vortex` ↔ `pandas-ta-classic.vortex` | `minus` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `Vortex` ↔ `pandas-ta-classic.vortex` | `plus` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 |  |
| `WeightedClose` ↔ `WCLPRICE` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `WeightedMovingAverage` ↔ `WMA` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `1.857e-10` | 0 | external parity plus bitwise lifecycle invariance |
| `WilliamsPercentR` ↔ `WILLR` | `all` | [TA-Lib](https://ta-lib.github.io/ta-lib-python/funcs.html) | `0.7.1` | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle invariance |
| `YangZhang` ↔ `self.yang_zhang` | `all` | native invariant | `repository invariant` | INVARIANT | `0.000e+00` | 0 | cold/warm/chunk/reset invariant; no external oracle |
| `ZeroLagExponentialMovingAverage` ↔ `pandas_ta_classic.zlma` | `zlema` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `7.105e-14` | 0 | force pandas-ta's native EMA; TA-Lib rejects leading ZLMA NaNs |
| `ZeroLagExponentialMovingAverage` ↔ `pandas_ta_classic.zlma` | `zlema[length=1]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `0.000e+00` | 0 | parameter matrix; force pandas-ta's native EMA |
| `ZeroLagExponentialMovingAverage` ↔ `pandas_ta_classic.zlma` | `zlema[length=21]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `7.105e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
| `ZeroLagExponentialMovingAverage` ↔ `pandas_ta_classic.zlma` | `zlema[length=2]` | [pandas-ta-classic](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | `0.6.52` | MATCH | `1.421e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
