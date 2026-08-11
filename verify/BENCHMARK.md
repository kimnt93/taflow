# TAFlow benchmark

Generated 2026-08-11 with Python 3.12.3, NumPy 2.4.6, TA-Lib 0.7.1, Wickra 0.9.9, pandas-ta-classic 0.6.52, SMC 0.0.27, and TAFlow 0.1.2.

Only `MATCH` indicators are timed. Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow. Each cell is API/kernel.

| Class | Target | 1k | 10k | 100k | 1m |
|---|---|---:|---:|---:|---:|
| AbsoluteBreadthIndex | Wickra `AbsoluteBreadthIndex` | 1300.29×/1608.52× | 2719.67×/3052.32× | 3498.87×/3598.28× | 2435.96×/3127.32× |
| AbsolutePriceOscillator | TA-Lib `APO` | 4.47×/4.85× | 1.49×/1.62× | 0.97×/1.01× | 0.95×/1.05× |
| AccelerationBands | TA-Lib `ACCBANDS` | 2.80×/3.48× | 1.14×/1.29× | 0.85×/0.87× | 0.72×/0.73× |
| AccumulationDistribution | TA-Lib `AD` | 2.15×/3.56× | 1.04×/1.14× | 0.52×/0.59× | 0.46×/0.56× |
| AccumulationDistributionOscillator | TA-Lib `ADOSC` | 2.49×/2.83× | 1.00×/0.62× | 0.29×/0.32× | 0.38×/0.40× |
| AdaptiveCycle | Wickra `AdaptiveCycle` | 2.26×/2.28× | 1.24×/1.30× | 1.30×/1.28× | 1.28×/1.32× |
| Amihud | Wickra `AmihudIlliquidity` | 43.29×/49.17× | 53.37×/55.53× | 56.97×/59.81× | 56.45×/60.09× |
| AnchoredVolumeWeightedAveragePrice | NumPy `anchored VWAP deviation bands` | 78.18×/103.04× | 151.27×/166.66× | 164.17×/170.22× | 146.75×/169.05× |
| ArmsIndex | Wickra `Trin` | 688.99×/897.71× | 1986.97×/2245.44× | 2237.64×/2482.20× | 1946.94×/2247.84× |
| ArnaudLegouxMovingAverage | Wickra `ALMA` | 16.08×/16.71× | 4.19×/4.28× | 3.18×/3.21× | 2.83×/2.93× |
| Aroon | TA-Lib `AROON` | 1.57×/1.45× | 0.53×/0.54× | 0.37×/0.42× | 0.37×/0.40× |
| AroonOscillator | TA-Lib `AROONOSC` | 1.24×/1.32× | 0.50×/0.50× | 0.38×/0.41× | 0.35×/0.36× |
| AutomaticFibonacci | Wickra `AutoFib` | 23.32×/28.50× | 22.38×/24.54× | 25.47×/30.56× | 24.69×/28.84× |
| AverageDailyDollarValue | NumPy `rolling average dollar volume` | 8.62×/9.53× | 5.23×/5.51× | 4.30×/4.73× | 4.26×/4.62× |
| AverageDailyRange | Wickra `AverageDailyRange` | 25.54×/31.89× | 28.94×/38.33× | 36.18×/43.33× | 44.19×/48.76× |
| AverageDirectionalIndex | TA-Lib `ADX` | 3.46×/4.23× | 1.41×/1.47× | 1.02×/1.08× | 0.60×/0.97× |
| AverageDirectionalIndexRating | TA-Lib `ADXR` | 2.23×/2.46× | 1.00×/1.08× | 1.13×/1.19× | 0.90×/0.89× |
| AveragePrice | TA-Lib `AVGPRICE` | 3.13×/3.89× | 0.92×/1.33× | 0.38×/0.44× | 0.52×/0.66× |
| AverageTrueRange | TA-Lib `ATR` | 3.36×/3.76× | 1.23×/1.25× | 0.95×/0.96× | 0.89×/0.98× |
| AverageTrueRangeBands | Wickra `AtrBands` | 34.32×/42.74× | 45.35×/50.23× | 46.44×/55.28× | 48.03×/59.90× |
| AwesomeOscillator | Wickra `AwesomeOscillator` | 8.39×/8.55× | 3.51×/3.52× | 2.86×/2.73× | 2.77×/2.84× |
| BalanceOfPower | TA-Lib `BOP` | 3.01×/3.90× | 0.98×/1.37× | 0.53×/0.61× | 0.66×/0.79× |
| BarsSince | NumPy `bars since condition` | 21.95×/25.57× | 36.99×/40.33× | 41.74×/46.35× | 44.46×/53.68× |
| BatPattern | Wickra `Bat` | 15.57×/18.74× | 14.46×/15.15× | 14.05×/14.57× | 13.12×/14.40× |
| BetterVolume | Wickra `BetterVolume` | 10.49×/11.96× | 8.45×/8.62× | 8.04×/8.33× | 7.56×/7.79× |
| BollingerBands | TA-Lib `BBANDS` | 7.68×/6.58× | 1.39×/2.35× | 1.36×/1.69× | 0.41×/1.37× |
| BreadthThrust | Wickra `BreadthThrust` | 839.24×/1001.89× | 1410.86×/1465.75× | 1513.48×/1311.14× | 1096.46×/1327.38× |
| BreakOfStructureChangeOfCharacter | NumPy `causal BOS and CHOCH events` | 80.12×/95.81× | 97.24×/101.88× | 106.98×/101.21× | 63.17×/90.27× |
| BullishPercentIndex | Wickra `BullishPercentIndex` | 1622.75×/2033.23× | 3831.00×/4151.03× | 4701.05×/4624.64× | 3714.22×/4319.55× |
| ButterflyPattern | Wickra `Butterfly` | 15.05×/19.42× | 14.12×/14.96× | 12.94×/13.98× | 13.51×/13.89× |
| CandleAbandonedBaby | TA-Lib `CDLABANDONEDBABY` | 1.84×/2.23× | 0.78×/0.87× | 0.71×/0.69× | 0.64×/0.62× |
| CandleAdvanceBlock | TA-Lib `CDLADVANCEBLOCK` | 2.10×/2.51× | 1.22×/1.25× | 1.22×/1.27× | 1.24×/1.43× |
| CandleBeltHold | TA-Lib `CDLBELTHOLD` | 1.78×/2.38× | 0.75×/0.87× | 0.60×/0.69× | 0.58×/0.68× |
| CandleBreakaway | TA-Lib `CDLBREAKAWAY` | 2.31×/3.25× | 1.07×/1.10× | 0.75×/0.78× | 0.74×/0.76× |
| CandleClosingMarubozu | TA-Lib `CDLCLOSINGMARUBOZU` | 1.60×/1.88× | 0.82×/0.86× | 0.69×/0.67× | 0.68×/0.72× |
| CandleConcealBabySwall | TA-Lib `CDLCONCEALBABYSWALL` | 1.79×/2.11× | 0.73×/0.74× | 0.52×/0.51× | 0.52×/0.54× |
| CandleCounterAttack | TA-Lib `CDLCOUNTERATTACK` | 1.90×/2.28× | 0.77×/0.79× | 0.64×/0.65× | 0.68×/0.71× |
| CandleDarkCloudCover | TA-Lib `CDLDARKCLOUDCOVER` | 2.05×/2.44× | 0.78×/0.89× | 0.66×/0.67× | 0.65×/0.64× |
| CandleDoji | TA-Lib `CDLDOJI` | 2.45×/3.69× | 1.17×/1.31× | 0.72×/0.67× | 0.72×/0.74× |
| CandleDojiStar | TA-Lib `CDLDOJISTAR` | 1.91×/2.42× | 0.84×/0.87× | 0.69×/0.59× | 0.66×/0.65× |
| CandleDragonflyDoji | TA-Lib `CDLDRAGONFLYDOJI` | 2.49×/3.36× | 1.25×/1.28× | 0.98×/0.97× | 0.90×/0.97× |
| CandleEngulfing | TA-Lib `CDLENGULFING` | 2.31×/3.21× | 1.09×/1.19× | 0.75×/0.78× | 0.72×/0.77× |
| CandleEveningDojiStar | TA-Lib `CDLEVENINGDOJISTAR` | 2.04×/2.30× | 0.77×/0.78× | 0.55×/0.56× | 0.53×/0.55× |
| CandleEveningStar | TA-Lib `CDLEVENINGSTAR` | 1.76×/2.12× | 0.75×/0.78× | 0.55×/0.58× | 0.60×/0.61× |
| CandleGapSideSideWhite | TA-Lib `CDLGAPSIDESIDEWHITE` | 2.14×/2.98× | 1.67×/1.74× | 1.61×/1.60× | 1.39×/1.44× |
| CandleGravestoneDoji | TA-Lib `CDLGRAVESTONEDOJI` | 2.79×/3.88× | 1.28×/1.27× | 0.99×/0.96× | 0.99×/0.99× |
| CandleHammer | TA-Lib `CDLHAMMER` | 2.54×/3.78× | 1.42×/1.47× | 1.16×/1.16× | 1.23×/1.23× |
| CandleHangingMan | TA-Lib `CDLHANGINGMAN` | 1.95×/2.65× | 0.99×/0.93× | 0.78×/0.78× | 0.80×/0.82× |
| CandleHarami | TA-Lib `CDLHARAMI` | 1.71×/2.02× | 0.95×/0.97× | 0.82×/0.83× | 0.80×/0.78× |
| CandleHaramiCross | TA-Lib `CDLHARAMICROSS` | 1.68×/2.21× | 0.93×/0.94× | 0.77×/0.77× | 0.75×/0.77× |
| CandleHighWave | TA-Lib `CDLHIGHWAVE` | 1.93×/2.52× | 1.02×/1.10× | 0.94×/0.97× | 0.93×/0.92× |
| CandleHikkake | TA-Lib `CDLHIKKAKE` | 3.02×/4.35× | 1.21×/1.31× | 0.76×/0.85× | 0.81×/0.87× |
| CandleHikkakeModified | TA-Lib `CDLHIKKAKEMOD` | 2.67×/2.58× | 1.36×/1.43× | 0.99×/1.04× | 0.90×/0.95× |
| CandleHomingPigeon | TA-Lib `CDLHOMINGPIGEON` | 1.59×/1.86× | 0.87×/0.97× | 0.67×/0.78× | 0.65×/0.58× |
| CandleIdenticalThreeCrows | TA-Lib `CDLIDENTICAL3CROWS` | 1.40×/1.65× | 0.74×/0.78× | 0.59×/0.67× | 0.57×/0.68× |
| CandleInNeck | TA-Lib `CDLINNECK` | 1.99×/1.76× | 1.07×/0.83× | 0.71×/0.68× | 0.65×/0.62× |
| CandleInvertedHammer | TA-Lib `CDLINVERTEDHAMMER` | 2.36×/2.95× | 1.01×/1.12× | 0.93×/0.96× | 0.88×/0.84× |
| CandleKicking | TA-Lib `CDLKICKING` | 1.48×/2.00× | 0.95×/0.99× | 0.96×/0.99× | 0.83×/0.87× |
| CandleKickingByLength | TA-Lib `CDLKICKINGBYLENGTH` | 1.98×/2.27× | 1.09×/1.17× | 1.02×/1.05× | 0.86×/0.93× |
| CandleLadderBottom | TA-Lib `CDLLADDERBOTTOM` | 1.81×/2.19× | 0.62×/0.67× | 0.46×/0.47× | 0.43×/0.44× |
| CandleLongLeggedDoji | TA-Lib `CDLLONGLEGGEDDOJI` | 2.33×/3.09× | 1.18×/1.32× | 0.82×/0.86× | 1.05×/1.02× |
| CandleLongLine | TA-Lib `CDLLONGLINE` | 1.93×/2.33× | 1.11×/1.18× | 1.01×/0.86× | 1.00×/0.90× |
| CandleMarubozu | TA-Lib `CDLMARUBOZU` | 1.76×/1.45× | 0.89×/0.92× | 0.76×/0.75× | 0.72×/0.75× |
| CandleMatHold | TA-Lib `CDLMATHOLD` | 1.51×/2.01× | 0.62×/0.66× | 0.50×/0.47× | 0.42×/0.40× |
| CandleMatchingLow | TA-Lib `CDLMATCHINGLOW` | 1.83×/2.12× | 0.85×/0.90× | 0.77×/0.52× | 0.59×/0.65× |
| CandleMorningDojiStar | TA-Lib `CDLMORNINGDOJISTAR` | 1.82×/2.34× | 0.73×/0.79× | 0.50×/0.55× | 0.57×/0.61× |
| CandleMorningStar | TA-Lib `CDLMORNINGSTAR` | 2.00×/2.40× | 0.83×/0.84× | 0.66×/0.69× | 0.57×/0.58× |
| CandleOnNeck | TA-Lib `CDLONNECK` | 1.65×/2.06× | 0.80×/0.84× | 0.63×/0.66× | 0.64×/0.65× |
| CandlePiercing | TA-Lib `CDLPIERCING` | 1.72×/2.27× | 0.87×/0.94× | 0.81×/0.77× | 0.73×/0.77× |
| CandleRickshawman | TA-Lib `CDLRICKSHAWMAN` | 1.69×/2.43× | 1.08×/0.68× | 0.83×/0.86× | 0.80×/0.82× |
| CandleRiseFallThreeMethods | TA-Lib `CDLRISEFALL3METHODS` | 1.18×/1.20× | 0.61×/0.63× | 0.51×/0.51× | 0.38×/0.54× |
| CandleSeparatingLines | TA-Lib `CDLSEPARATINGLINES` | 1.88×/2.29× | 1.10×/1.04× | 0.73×/0.77× | 0.77×/0.76× |
| CandleShootingStar | TA-Lib `CDLSHOOTINGSTAR` | 2.38×/2.89× | 1.07×/1.17× | 0.85×/0.88× | 0.92×/0.83× |
| CandleShortLine | TA-Lib `CDLSHORTLINE` | 2.08×/2.49× | 1.24×/1.33× | 1.02×/1.10× | 1.02×/1.08× |
| CandleSpinningTop | TA-Lib `CDLSPINNINGTOP` | 2.88×/2.67× | 0.89×/1.15× | 0.91×/0.93× | 0.86×/0.89× |
| CandleStalledPattern | TA-Lib `CDLSTALLEDPATTERN` | 1.87×/2.26× | 0.99×/0.92× | 0.68×/0.84× | 0.83×/0.84× |
| CandleStickSandwich | TA-Lib `CDLSTICKSANDWICH` | 2.96×/4.24× | 1.63×/1.77× | 1.08×/1.10× | 0.97×/1.01× |
| CandleTakuri | TA-Lib `CDLTAKURI` | 1.77×/2.31× | 1.07×/1.15× | 0.72×/0.72× | 0.67×/0.67× |
| CandleTasukiGap | TA-Lib `CDLTASUKIGAP` | 2.13×/2.62× | 1.19×/1.17× | 0.96×/0.97× | 0.91×/0.95× |
| CandleThreeBlackCrows | TA-Lib `CDL3BLACKCROWS` | 2.22×/3.32× | 1.36×/1.49× | 0.85×/0.86× | 0.84×/0.86× |
| CandleThreeInside | TA-Lib `CDL3INSIDE` | 2.16×/3.04× | 1.60×/1.74× | 0.83×/0.98× | 1.12×/1.16× |
| CandleThreeLineStrike | TA-Lib `CDL3LINESTRIKE` | 2.52×/3.32× | 1.36×/1.75× | 1.02×/1.05× | 1.18×/1.12× |
| CandleThreeOutside | TA-Lib `CDL3OUTSIDE` | 2.29×/3.79× | 1.23×/1.33× | 0.74×/0.77× | 0.70×/0.71× |
| CandleThreeStarsInSouth | TA-Lib `CDL3STARSINSOUTH` | 2.49×/2.52× | 1.82×/1.99× | 1.32×/1.28× | 1.18×/1.09× |
| CandleThreeWhiteSoldiers | TA-Lib `CDL3WHITESOLDIERS` | 2.83×/3.11× | 1.06×/1.11× | 1.20×/1.19× | 0.90×/0.94× |
| CandleThrusting | TA-Lib `CDLTHRUSTING` | 1.53×/2.23× | 0.87×/0.88× | 0.60×/0.64× | 0.63×/0.65× |
| CandleTriStar | TA-Lib `CDLTRISTAR` | 1.94×/2.31× | 0.76×/0.82× | 0.62×/0.66× | 0.55×/0.59× |
| CandleTwoCrows | TA-Lib `CDL2CROWS` | 2.43×/3.62× | 1.51×/1.68× | 1.11×/1.09× | 0.99×/0.95× |
| CandleUniqueThreeRiver | TA-Lib `CDLUNIQUE3RIVER` | 1.83×/2.10× | 0.71×/0.61× | 0.40×/0.44× | 0.44×/0.43× |
| CandleUpDownSideGapThreeMethods | TA-Lib `CDLXSIDEGAP3METHODS` | 2.00×/2.33× | 0.71×/0.72× | 0.56×/0.57× | 0.55×/0.56× |
| CandleUpsideGapTwoCrows | TA-Lib `CDLUPSIDEGAP2CROWS` | 1.89×/2.04× | 0.79×/0.96× | 0.76×/0.77× | 0.74×/0.70× |
| CenterOfGravity | Wickra `CenterOfGravity` | 8.40×/8.49× | 3.15×/3.39× | 3.01×/3.09× | 2.67×/2.89× |
| ChaikinMoneyFlow | Wickra `ChaikinMoneyFlow` | 18.07×/24.05× | 20.67×/20.27× | 20.77×/22.03× | 19.46×/21.63× |
| ChaikinVolatility | Wickra `ChaikinVolatility` | 21.53×/22.71× | 10.83×/11.16× | 9.61×/9.81× | 9.17×/9.61× |
| ChandeMomentumOscillator | TA-Lib `CMO` | 4.49×/5.04× | 1.60×/1.62× | 1.06×/1.09× | 0.92×/1.05× |
| CloseToCloseSigma | NumPy `annualized close-to-close volatility` | 6.47×/6.69× | 3.49×/3.64× | 3.46×/3.79× | 3.96×/3.99× |
| CommodityChannelIndex | TA-Lib `CCI` | 2.22×/2.27× | 1.17×/1.19× | 1.08×/1.08× | 1.03×/1.07× |
| CrabPattern | Wickra `Crab` | 16.06×/18.90× | 15.32×/16.03× | 14.98×/11.48× | 12.40×/13.46× |
| Cross | NumPy `causal cross event` | 2.30×/2.62× | 0.84×/0.92× | 0.54×/0.60× | 1.23×/1.38× |
| Crossover | NumPy `causal crossover` | 2.08×/2.55× | 0.73×/0.84× | 0.47×/0.50× | 0.82×/1.05× |
| Crossunder | NumPy `causal crossunder` | 2.18×/2.59× | 0.78×/0.86× | 0.55×/0.60× | 0.70×/0.87× |
| CumulativeCount | NumPy `one-based cumulative count` | 3.69×/4.89× | 1.95×/2.70× | 0.90×/1.38× | 0.62×/1.21× |
| CumulativeMaximum | NumPy `numpy.maximum.accumulate` | 2.73×/3.31× | 1.28×/1.39× | 0.94×/1.03× | 0.82×/0.88× |
| CumulativeMinimum | NumPy `numpy.minimum.accumulate` | 2.94×/3.47× | 1.24×/1.34× | 0.95×/1.05× | 0.89×/1.01× |
| CumulativeProduct | NumPy `numpy.cumprod` | 3.43×/4.36× | 2.69×/3.05× | 1.89×/2.31× | 1.23×/2.03× |
| CumulativeSum | NumPy `numpy.cumsum` | 3.71×/5.03× | 2.42×/2.87× | 1.87×/2.48× | 1.19×/1.74× |
| CumulativeSumControlChart | NumPy `CUSUM event filter` | 75.33×/82.22× | 121.92×/129.18× | 126.49×/135.32× | 119.40×/135.52× |
| CumulativeVolumeIndex | Wickra `CumulativeVolumeIndex` | 634.15×/778.08× | 1364.70×/1545.69× | 1606.53×/1178.08× | 1501.93×/1695.27× |
| CupAndHandle | Wickra `CupAndHandle` | 17.27×/21.71× | 18.66×/19.31× | 15.51×/15.65× | 14.52×/14.89× |
| CypherPattern | Wickra `Cypher` | 17.23×/22.20× | 17.02×/17.16× | 15.14×/15.75× | 16.09×/17.05× |
| DayOfWeekReturnProfile | Wickra `DayOfWeekProfile` | 20.68×/24.27× | 22.70×/24.56× | 23.43×/27.29× | 13.54×/19.11× |
| DecayLinear | NumPy `linear decay weighted mean` | 12.84×/14.29× | 7.21×/7.82× | 6.20×/6.67× | 5.84×/6.62× |
| Decycler | Wickra `Decycler` | 14.23×/16.25× | 6.98×/6.94× | 5.73×/6.09× | 5.26×/6.01× |
| DecyclerOscillator | Wickra `DecyclerOscillator` | 17.04×/18.04× | 7.25×/7.10× | 4.99×/5.07× | 4.41×/5.00× |
| DemandIndex | Wickra `DemandIndex` | 18.83×/25.38× | 22.42×/24.78× | 24.58×/26.01× | 20.11×/25.19× |
| DetrendedPriceOscillator | pandas-ta-classic `dpo` | 43.57×/47.84× | 8.15×/8.71× | 2.70×/2.87× | 2.83×/3.15× |
| DirectionalMovementIndex | TA-Lib `DX` | 2.33×/2.69× | 1.09×/1.15× | 0.84×/0.87× | 0.81×/0.86× |
| Donchian | Wickra `Donchian` | 51.89×/62.95× | 46.75×/44.18× | 55.53×/60.83× | 22.68×/56.09× |
| DoubleBollingerBands | Wickra `DoubleBollinger` | 15.82×/16.43× | 11.27×/12.09× | 11.93×/12.90× | 13.00×/13.91× |
| DoubleExponentialMovingAverage | TA-Lib `DEMA` | 4.02×/4.44× | 1.52×/1.59× | 1.17×/1.23× | 1.21×/1.32× |
| Drawdown | NumPy `drawdown from cumulative maximum` | 3.16×/3.48× | 1.33×/1.41× | 1.03×/1.10× | 1.22×/1.41× |
| EaseOfMovement | Wickra `EaseOfMovement` | 15.26×/24.18× | 17.13×/17.11× | 15.11×/14.64× | 17.22×/18.39× |
| EhlersStochastic | Wickra `EhlersStochastic` | 4.60×/4.63× | 2.10×/2.14× | 1.75×/1.85× | 1.82×/1.74× |
| EmpiricalModeDecomposition | Wickra `EmpiricalModeDecomposition` | 7.59×/7.90× | 2.23×/2.23× | 1.83×/1.85× | 1.80×/1.71× |
| EntryExit | NumPy `entry-exit position state` | 19.52×/24.09× | 43.47×/49.84× | 52.61×/60.10× | 53.34×/60.76× |
| EqualHighsLows | NumPy `causal equal pivot levels` | 99.77×/104.93× | 102.86×/105.17× | 107.87×/112.57× | 99.53×/104.19× |
| EvenBetterSinewave | pandas-ta-classic `ebsw` | 1414.17×/1591.55× | 1917.35×/1972.31× | 1964.87×/1925.59× | 1940.30×/2237.18× |
| ExponentialMovingAverage | TA-Lib `EMA` | 6.66×/7.30× | 2.12×/2.08× | 1.05×/1.18× | 0.71×/0.75× |
| ExponentiallyWeightedCorrelation | NumPy `ewm correlation` | 123.95×/139.59× | 197.76×/224.34× | 214.42×/223.28× | 225.93×/247.22× |
| ExponentiallyWeightedCovariance | NumPy `ewm covariance` | 128.01×/148.82× | 215.22×/239.96× | 245.94×/195.11× | 222.13×/264.05× |
| ExponentiallyWeightedStandardDeviation | NumPy `ewm standard deviation` | 260.96×/295.39× | 247.38×/262.29× | 245.94×/312.09× | 262.47×/296.39× |
| ExponentiallyWeightedSum | NumPy `exponentially weighted sum` | 31.23×/34.71× | 46.15×/49.54× | 51.87×/54.16× | 48.74×/53.80× |
| ExponentiallyWeightedVariance | NumPy `ewm variance` | 183.08×/205.15× | 267.85×/291.72× | 301.02×/295.39× | 299.85×/326.82× |
| FairValueGap | SMC `smartmoneyconcepts.smc.fvg` | 174.16×/210.86× | 80.94×/89.94× | 60.32×/59.23× | 21.97×/61.63× |
| Falling | NumPy `period-over-period falling` | 4.21×/4.50× | 0.76×/0.81× | 0.32×/0.34× | 0.41×/0.46× |
| FastStochasticOscillator | TA-Lib `STOCHF` | 2.27×/2.28× | 0.82×/0.83× | 0.55×/0.56× | 0.61×/0.62× |
| FibonacciArcs | Wickra `FibArcs` | 15.93×/26.73× | 23.26×/25.02× | 25.96×/28.20× | 28.65×/32.04× |
| FibonacciChannel | Wickra `FibChannel` | 13.88×/29.67× | 25.57×/27.53× | 27.25×/32.96× | 27.05×/28.65× |
| FibonacciConfluence | Wickra `FibConfluence` | 3.59×/3.42× | 3.44×/3.44× | 3.40×/3.41× | 3.47×/3.46× |
| FibonacciExtension | Wickra `FibExtension` | 28.22×/33.34× | 26.52×/28.17× | 30.98×/35.30× | 29.38×/31.90× |
| FibonacciFan | Wickra `FibFan` | 28.31×/30.64× | 27.43×/29.33× | 30.00×/31.12× | 29.41×/33.94× |
| FibonacciProjection | Wickra `FibProjection` | 30.11×/34.16× | 30.08×/32.89× | 32.64×/35.10× | 29.70×/37.48× |
| FibonacciRetracement | NumPy `rolling Fibonacci levels` | 350.78×/434.47× | 327.05×/383.86× | 339.61×/400.79× | 173.40×/372.52× |
| FibonacciTimeZones | Wickra `FibTimeZones` | 29.18×/31.52× | 22.16×/24.05× | 26.16×/27.26× | 25.69×/28.14× |
| FisherTransform | pandas-ta-classic `fisher` | 31.65×/35.07× | 3.06×/3.84× | 1.63×/1.77× | 1.75×/1.86× |
| FlagPennant | Wickra `FlagPennant` | 17.81×/22.22× | 16.96×/17.73× | 15.11×/16.20× | 15.22×/16.89× |
| ForceIndex | Wickra `ForceIndex` | 20.04×/25.92× | 15.50×/16.06× | 14.14×/15.12× | 13.17×/13.72× |
| FourPointHarmonicPattern | Wickra `Abcd` | 16.11×/19.37× | 14.86×/15.76× | 14.54×/14.91× | 14.05×/15.64× |
| FracDiff | NumPy `fixed-width fractional differencing` | 3.86×/3.94× | 1.09×/1.10× | 1.07×/1.02× | 1.02×/1.03× |
| FractalDimension | NumPy `two-chunk rescaled-range dimension` | 3.35×/3.59× | 2.44×/2.48× | 2.88×/2.88× | 2.92×/2.96× |
| GapDown | NumPy `gap down relation` | 2.96×/3.84× | 1.30×/1.44× | 0.70×/0.85× | 1.37×/1.59× |
| GapUp | NumPy `gap up relation` | 3.07×/3.61× | 1.36×/1.52× | 0.83×/1.00× | 1.68×/1.89× |
| GarmanKlass | Wickra `GarmanKlassVolatility` | 12.46×/13.58× | 8.43×/9.10× | 8.71×/8.92× | 8.47×/8.88× |
| GarmanKlassYangZhang | NumPy `annualized Garman-Klass-Yang-Zhang volatility` | 3.88×/4.36× | 2.06×/2.11× | 1.84×/1.77× | 1.82×/1.86× |
| GartleyPattern | Wickra `Gartley` | 15.35×/17.84× | 13.39×/14.39× | 14.67×/15.10× | 12.84×/14.45× |
| GoldenPocket | Wickra `GoldenPocket` | 25.85×/29.99× | 27.95×/29.26× | 29.03×/29.75× | 30.34×/34.48× |
| HeadAndShoulders | Wickra `HeadAndShoulders` | 15.62×/18.71× | 12.79×/14.67× | 14.33×/14.65× | 12.76×/13.16× |
| HedgeRatio | NumPy `rolling OLS hedge ratio` | 6.26×/6.38× | 3.94×/3.99× | 5.20×/5.14× | 5.29×/5.59× |
| HeikinAshi | Wickra `HeikinAshi` | 41.17×/47.88× | 53.69×/59.91× | 66.75×/73.26× | 21.53×/74.93× |
| HighLowIndex | Wickra `HighLowIndex` | 860.96×/996.55× | 1371.35×/1438.23× | 1480.56×/1521.48× | 1360.32×/1421.62× |
| HigherHigh | NumPy `higher high relation` | 2.27×/2.54× | 0.74×/0.84× | 0.43×/0.47× | 0.52×/0.65× |
| HighestSince | NumPy `highest since condition` | 38.03×/45.56× | 66.12×/72.12× | 74.72×/69.51× | 76.05×/60.45× |
| HilbertDominantCycle | Wickra `HilbertDominantCycle` | 2.87×/2.96× | 1.60×/1.66× | 1.44×/1.50× | 1.42×/1.52× |
| HilbertTransformDominantCyclePeriod | TA-Lib `HT_DCPERIOD` | 1.62×/1.49× | 1.03×/1.03× | 0.93×/0.98× | 0.89×/0.93× |
| HilbertTransformDominantCyclePhase | TA-Lib `HT_DCPHASE` | 4.76×/4.92× | 4.16×/4.29× | 4.30×/4.19× | 3.93×/4.00× |
| HilbertTransformPhasor | TA-Lib `HT_PHASOR` | 1.65×/1.57× | 0.94×/0.97× | 0.97×/0.93× | 0.91×/0.91× |
| HilbertTransformSineWave | TA-Lib `HT_SINE` | 3.66×/3.68× | 3.31×/3.45× | 3.42×/3.55× | 3.38×/3.41× |
| HilbertTransformTrendMode | TA-Lib `HT_TRENDMODE` | 2.85×/2.90× | 2.76×/2.17× | 2.99×/2.73× | 2.76×/2.89× |
| HilbertTransformTrendline | TA-Lib `HT_TRENDLINE` | 1.14×/1.11× | 0.84×/0.87× | 0.80×/0.82× | 0.82×/0.84× |
| HullMovingAverage | Wickra `HMA` | 6.31×/6.65× | 2.36×/2.60× | 2.12×/2.07× | 1.94×/2.09× |
| Hurst | Wickra `HurstExponent` | 1.22×/1.19× | 0.86×/0.87× | 0.79×/0.77× | 0.80×/0.80× |
| HurstChannel | Wickra `HurstChannel` | 13.81×/14.72× | 11.51×/11.89× | 12.62×/13.14× | 13.57×/14.07× |
| Ichimoku | NumPy `causal ichimoku components` | 4.69×/5.09× | 2.78×/2.90× | 2.62×/2.58× | 2.07×/2.71× |
| InsideBar | NumPy `inside bar relation` | 3.19×/3.91× | 1.15×/1.38× | 0.90×/0.98× | 1.40×/1.65× |
| InstantaneousTrendline | Wickra `InstantaneousTrendline` | 15.84×/18.66× | 8.11×/8.51× | 6.77×/6.71× | 6.97×/4.78× |
| IntradayIntensity | Wickra `IntradayIntensity` | 21.24×/29.04× | 34.32×/42.77× | 62.55×/42.61× | 35.51×/40.86× |
| IntradayMomentumIndex | TA-Lib `IMI` | 5.12×/4.97× | 4.85×/5.02× | 4.46×/4.62× | 4.77×/4.96× |
| IntradayVolatilityProfile | Wickra `IntradayVolatilityProfile` | 25.72×/28.02× | 24.90×/27.74× | 28.61×/36.19× | 11.25×/18.45× |
| InverseFisherTransform | Wickra `InverseFisherTransform` | 25.67×/27.76× | 11.65×/12.17× | 9.32×/10.22× | 9.36×/10.35× |
| JurikMovingAverage | pandas-ta-classic `jma` | 231.09×/230.06× | 229.83×/220.20× | 166.07×/218.20× | 217.50×/219.89× |
| KalmanHedgeRatio | Wickra `KalmanHedgeRatio` | 29.51×/32.64× | 26.06×/27.87× | 31.72×/32.73× | 33.98×/35.76× |
| KaufmanAdaptiveMovingAverage | TA-Lib `KAMA` | 5.04×/5.41× | 1.61×/1.75× | 0.72×/1.07× | 0.99×/1.01× |
| KeltnerChannels | Wickra `Keltner` | 27.65×/34.11× | 32.06×/33.33× | 33.50×/38.45× | 21.40×/36.67× |
| KlingerVolumeOscillator | Wickra `KVO` | 11.46×/12.93× | 7.60×/7.92× | 7.06×/6.83× | 6.86×/7.38× |
| KnowSureThing | Wickra `KST` | 34.17×/36.30× | 21.43×/22.00× | 21.65×/22.95× | 22.76×/25.27× |
| Lag | NumPy `causal lag` | 4.44×/5.33× | 1.02×/1.07× | 0.28×/0.31× | 0.34×/0.41× |
| LaguerreRelativeStrengthIndex | Wickra `LaguerreRSI` | 18.11×/18.23× | 6.86×/7.44× | 5.83×/5.39× | 5.50×/5.55× |
| LinearRegressionChannel | Wickra `LinRegChannel` | 8.24×/8.71× | 7.05×/7.24× | 6.44×/6.51× | 7.34×/7.27× |
| Liquidity | NumPy `causal liquidity pools` | 117.07×/126.54× | 159.40×/153.44× | 252.52×/259.53× | 227.25×/295.37× |
| LogReturn | Wickra `LogReturn` | 24.01×/16.32× | 7.05×/6.32× | 5.83×/5.83× | 4.96×/5.31× |
| LowerLow | NumPy `lower low relation` | 3.04×/3.23× | 1.23×/1.33× | 0.85×/0.78× | 1.42×/1.63× |
| LowestSince | NumPy `lowest since condition` | 37.73×/36.57× | 67.56×/75.29× | 71.95×/53.93× | 42.58×/49.42× |
| MarketFacilitationIndex | Wickra `MarketFacilitationIndex` | 21.38×/28.43× | 37.79×/44.49× | 46.12×/51.99× | 34.16×/44.27× |
| MassIndex | Wickra `MassIndex` | 19.20×/20.63× | 8.66×/9.25× | 11.87×/12.88× | 7.14×/7.73× |
| MathAbs | NumPy `numpy.abs` | 3.44×/4.48× | 1.98×/2.93× | 0.80×/1.39× | 0.72×/1.15× |
| MathAcos | TA-Lib `ACOS` | 2.75×/4.96× | 1.34×/1.84× | 0.84×/0.91× | 1.51×/1.66× |
| MathAcosh | NumPy `numpy.arccosh` | 1.60×/1.74× | 1.05×/1.11× | 0.98×/1.02× | 0.95×/1.00× |
| MathAdd | TA-Lib `ADD` | 6.00×/7.97× | 3.21×/4.70× | 1.03×/1.69× | 0.81×/0.92× |
| MathAsin | TA-Lib `ASIN` | 2.74×/4.00× | 1.29×/1.42× | 0.96×/1.04× | 0.86×/0.99× |
| MathAsinh | NumPy `numpy.arcsinh` | 1.43×/1.63× | 0.83×/1.06× | 1.01×/1.05× | 0.93×/0.99× |
| MathAtan | TA-Lib `ATAN` | 3.53×/3.83× | 1.39×/1.36× | 1.06×/1.13× | 1.09×/1.16× |
| MathAtanh | NumPy `numpy.arctanh` | 1.99×/2.04× | 1.33×/1.38× | 1.27×/1.28× | 1.06×/1.17× |
| MathCbrt | NumPy `numpy.cbrt` | 1.30×/1.38× | 0.88×/0.90× | 0.83×/0.85× | 0.81×/0.83× |
| MathCeil | TA-Lib `CEIL` | 5.61×/6.81× | 1.53×/1.67× | 0.82×/0.84× | 0.55×/0.76× |
| MathCos | TA-Lib `COS` | 3.08×/3.39× | 1.18×/1.25× | 0.89×/1.03× | 0.99×/1.04× |
| MathCosh | TA-Lib `COSH` | 3.67×/4.16× | 1.28×/1.42× | 1.02×/1.10× | 0.92×/0.97× |
| MathCot | NumPy `numpy.tan reciprocal` | 1.69×/1.81× | 1.10×/1.12× | 0.98×/1.01× | 1.27×/1.25× |
| MathDegrees | NumPy `numpy.degrees` | 3.79×/5.10× | 3.25×/5.18× | 2.47×/4.38× | 1.60×/2.67× |
| MathDivide | TA-Lib `DIV` | 6.00×/8.83× | 3.07×/4.07× | 0.92×/1.60× | 0.62×/0.98× |
| MathExp | TA-Lib `EXP` | 3.16×/4.23× | 1.31×/1.05× | 1.20×/1.25× | 0.86×/0.97× |
| MathFloor | TA-Lib `FLOOR` | 4.93×/6.09× | 1.75×/1.64× | 0.70×/0.72× | 0.71×/0.91× |
| MathLn | TA-Lib `LN` | 4.12×/4.68× | 1.35×/1.46× | 0.94×/0.86× | 0.92×/0.95× |
| MathLog10 | TA-Lib `LOG10` | 2.98×/3.54× | 1.14×/1.25× | 0.94×/0.99× | 0.86×/0.96× |
| MathLog1p | NumPy `numpy.log1p` | 1.80×/1.45× | 1.13×/1.18× | 0.97×/1.02× | 0.77×/1.03× |
| MathMultiply | TA-Lib `MULT` | 6.13×/8.46× | 3.27×/4.60× | 1.00×/1.59× | 0.78×/1.05× |
| MathRadians | NumPy `numpy.radians` | 4.02×/5.22× | 3.16×/5.07× | 2.32×/4.17× | 1.51×/2.60× |
| MathSin | TA-Lib `SIN` | 2.55×/3.08× | 1.17×/0.98× | 1.00×/1.03× | 0.99×/0.98× |
| MathSinh | TA-Lib `SINH` | 3.61×/3.70× | 1.38×/1.47× | 0.96×/1.07× | 0.95×/1.07× |
| MathSqrt | TA-Lib `SQRT` | 7.08×/9.25× | 3.37×/4.29× | 1.75×/2.25× | 1.16×/1.71× |
| MathSubtract | TA-Lib `SUB` | 5.53×/7.69× | 3.85×/5.72× | 1.04×/1.66× | 0.62×/1.00× |
| MathTan | TA-Lib `TAN` | 2.28×/2.51× | 0.71×/1.13× | 1.03×/1.05× | 0.99×/1.01× |
| MathTanh | TA-Lib `TANH` | 5.61×/6.83× | 1.68×/1.79× | 1.25×/1.37× | 0.94×/0.91× |
| McClellanOscillator | Wickra `McClellanOscillator` | 931.23×/1106.52× | 1614.95×/1727.48× | 1638.52×/1937.30× | 1530.59×/1799.79× |
| McClellanSummationIndex | Wickra `McClellanSummationIndex` | 867.46×/998.67× | 1507.34×/1620.68× | 1687.96×/1751.04× | 1532.23×/1589.24× |
| McGinleyDynamic | Wickra `McGinleyDynamic` | 11.45×/12.35× | 4.46×/4.80× | 3.75×/3.77× | 3.72×/3.75× |
| MedianChannel | Wickra `MedianChannel` | 5.77×/5.59× | 4.65×/4.57× | 4.61×/4.68× | 5.03×/4.99× |
| MedianPrice | TA-Lib `MEDPRICE` | 4.22×/5.62× | 1.74×/1.92× | 0.45×/0.51× | 0.57×/0.78× |
| MesaAdaptiveMovingAverage | TA-Lib `MAMA` | 1.39×/1.41× | 0.99×/1.10× | 0.93×/0.97× | 0.90×/0.97× |
| MinusDirectionalIndicator | TA-Lib `MINUS_DI` | 2.58×/2.83× | 0.74×/1.06× | 0.70×/0.58× | 0.69×/0.66× |
| MinusDirectionalMovement | TA-Lib `MINUS_DM` | 4.49×/4.95× | 1.48×/1.53× | 1.16×/1.16× | 1.06×/1.14× |
| Momentum | TA-Lib `MOM` | 6.24×/7.08× | 1.94×/1.79× | 0.30×/0.38× | 0.40×/0.53× |
| MoneyFlowIndex | TA-Lib `MFI` | 3.55×/4.09× | 1.88×/1.94× | 1.21×/1.41× | 1.19×/1.23× |
| MovingAverage | TA-Lib `MA` | 6.45×/7.64× | 2.04×/2.46× | 1.03×/1.18× | 0.88×/1.00× |
| MovingAverageConvergenceDivergence | TA-Lib `MACD` | 8.31×/10.11× | 3.82×/3.74× | 2.89×/4.28× | 0.83×/3.85× |
| MovingAverageConvergenceDivergenceExtended | TA-Lib `MACDEXT` | 3.91×/4.22× | 1.01×/1.06× | 0.60×/0.72× | 0.40×/0.46× |
| MovingAverageConvergenceDivergenceFixed | TA-Lib `MACDFIX` | 8.51×/10.83× | 4.05×/5.23× | 3.46×/4.54× | 0.83×/4.37× |
| MovingAverageEnvelope | Wickra `MaEnvelope` | 23.60×/22.37× | 26.91×/28.99× | 19.63×/24.65× | 21.11×/23.65× |
| NegativeVolumeIndex | Wickra `NVI` | 22.60×/27.81× | 12.85×/13.91× | 13.19×/11.44× | 10.99×/12.34× |
| NewHighsNewLows | Wickra `NewHighsNewLows` | 1172.61×/1417.81× | 2713.84×/3006.90× | 3453.17×/3897.83× | 2666.53×/2332.24× |
| NormalizedAverageTrueRange | TA-Lib `NATR` | 3.48×/4.00× | 0.65×/1.00× | 0.78×/0.90× | 0.82×/0.87× |
| OnBalanceVolume | TA-Lib `OBV` | 3.66×/3.92× | 1.02×/1.06× | 0.64×/0.69× | 0.56×/0.59× |
| OpeningRange | NumPy `anchored opening range` | 36.80×/48.69× | 73.75×/88.16× | 75.04×/98.56× | 66.89×/91.54× |
| OrderBlock | NumPy `causal dual-scale order blocks` | 111.41×/117.66× | 136.79×/138.26× | 137.69×/134.37× | 116.87×/123.71× |
| OrnsteinUhlenbeckHalfLife | NumPy `rolling OU half life` | 5.73×/5.58× | 3.68×/3.60× | 5.02×/4.51× | 4.29×/4.18× |
| OutsideBar | NumPy `outside bar relation` | 3.03×/3.48× | 1.24×/1.37× | 0.76×/0.88× | 1.49×/1.73× |
| OvernightGap | Wickra `OvernightGap` | 25.16×/33.44× | 44.83×/53.81× | 53.83×/56.22× | 54.32×/59.36× |
| OvernightIntradayReturn | Wickra `OvernightIntradayReturn` | 21.28×/32.18× | 80.17×/81.81× | 70.77×/82.66× | 73.92×/81.35× |
| ParabolicMovingAverageStop | pandas-ta-classic `pmax` | 116.14×/134.11× | 86.86×/88.21× | 87.44×/89.67× | 80.58×/85.33× |
| ParabolicSar | TA-Lib `SAR` | 3.11×/3.27× | 0.88×/0.93× | 0.58×/0.60× | 0.53×/0.56× |
| ParabolicSarExtended | TA-Lib `SAREXT` | 4.26×/4.64× | 0.91×/0.91× | 0.57×/0.58× | 0.52×/0.54× |
| Parkinson | Wickra `ParkinsonVolatility` | 11.05×/12.55× | 6.49×/6.49× | 5.54×/5.95× | 5.30×/5.47× |
| PercentAboveMovingAverage | Wickra `PercentAboveMa` | 1695.41×/2100.15× | 3754.09×/4313.38× | 4626.29×/5146.90× | 3789.50×/4646.07× |
| PercentagePriceOscillator | TA-Lib `PPO` | 4.85×/5.60× | 1.77×/1.64× | 1.31×/1.42× | 1.16×/1.32× |
| PivotPoints | NumPy `anchored classic pivot points` | 46.75×/61.13× | 86.25×/98.96× | 85.90×/102.20× | 30.48×/88.73× |
| PlusDirectionalIndicator | TA-Lib `PLUS_DI` | 3.12×/3.81× | 1.44×/1.53× | 1.05×/1.24× | 0.97×/1.10× |
| PlusDirectionalMovement | TA-Lib `PLUS_DM` | 3.60×/4.64× | 1.62×/1.56× | 0.99×/1.06× | 0.85×/0.96× |
| PositionHold | NumPy `nonzero position hold` | 23.58×/27.73× | 47.71×/51.21× | 51.36×/57.71× | 47.53×/57.50× |
| PositiveVolumeIndex | Wickra `PVI` | 18.79×/21.73× | 12.19×/12.68× | 16.20×/14.76× | 10.72×/11.16× |
| PremiumDiscount | NumPy `rolling premium-discount zone` | 147.62×/155.48× | 121.48×/127.13× | 132.07×/119.23× | 119.82×/127.97× |
| PreviousHighLow | NumPy `previous-session high-low` | 35.86×/43.28× | 54.87×/61.02× | 60.11×/61.64× | 55.87×/67.56× |
| ProjectionBands | NumPy `rolling projection mean` | 4.88×/4.94× | 1.94×/1.97× | 1.56×/1.60× | 1.57×/1.58× |
| QuartileBands | Wickra `QuartileBands` | 9.53×/9.55× | 7.48×/7.52× | 7.61×/7.67× | 8.41×/7.10× |
| RateOfChange | TA-Lib `ROC` | 7.14×/7.92× | 2.05×/2.23× | 0.73×/0.81× | 0.55×/0.67× |
| RateOfChangePercent | TA-Lib `ROCP` | 6.36×/6.14× | 1.63×/1.79× | 0.69×/0.71× | 0.47×/0.65× |
| RateOfChangeRatio | TA-Lib `ROCR` | 4.54×/7.85× | 1.80×/1.99× | 0.68×/0.72× | 0.50×/0.70× |
| RateOfChangeRatioPercent | TA-Lib `ROCR100` | 5.95×/7.14× | 2.00×/2.24× | 0.66×/0.80× | 0.67×/0.87× |
| RectangleRange | Wickra `RectangleRange` | 16.45×/17.70× | 14.96×/11.98× | 13.79×/13.83× | 13.16×/13.35× |
| RelativeMomentumIndex | Wickra `RMI` | 19.06×/19.99× | 7.14×/7.83× | 6.23×/6.43× | 5.64×/6.05× |
| RelativeStrengthIndex | TA-Lib `RSI` | 3.58×/3.64× | 1.09×/1.14× | 0.79×/0.77× | 0.70×/0.76× |
| Retracements | NumPy `causal swing retracements` | 92.93×/108.27× | 114.52×/118.70× | 119.05×/124.82× | 115.34×/120.84× |
| Rising | NumPy `period-over-period rising` | 3.97×/4.31× | 0.77×/0.82× | 0.26×/0.28× | 0.34×/0.38× |
| RogersSatchell | Wickra `RogersSatchellVolatility` | 7.72×/8.46× | 5.65×/5.84× | 5.51×/5.56× | 5.15×/5.39× |
| RollSpread | NumPy `rolling Roll spread estimator` | 4.36×/5.53× | 2.78×/2.91× | 2.99×/2.76× | 3.34×/3.38× |
| RollingAlpha | Wickra `Alpha` | 5.18×/5.53× | 2.32×/2.38× | 1.98×/2.13× | 2.03×/1.78× |
| RollingAutocorr | Wickra `Autocorrelation` | 4.24×/3.96× | 1.82×/1.82× | 1.66×/1.60× | 1.89×/1.88× |
| RollingAverageDeviation | TA-Lib `AVGDEV` | 2.26×/2.41× | 1.08×/1.13× | 0.85×/0.85× | 0.86×/0.93× |
| RollingAverageDrawdown | Wickra `AverageDrawdown` | 4.13×/4.19× | 2.15×/2.12× | 1.74×/1.78× | 1.79×/1.79× |
| RollingBeta | TA-Lib `BETA` | 4.08×/4.60× | 1.87×/1.99× | 0.75×/1.31× | 1.07×/1.19× |
| RollingBetaNeutralSpread | Wickra `BetaNeutralSpread` | 4.01×/2.90× | 2.08×/2.05× | 1.61×/1.65× | 1.78×/1.73× |
| RollingCalmar | NumPy `rolling calmar on equity` | 5.52×/5.68× | 3.23×/3.40× | 3.62×/3.80× | 4.00×/3.85× |
| RollingCoefficientOfDetermination | NumPy `rolling squared correlation` | 6.04×/6.18× | 4.15×/4.17× | 4.68×/4.83× | 5.18×/5.20× |
| RollingCointegration | Wickra `Cointegration` | 9.84×/9.81× | 9.91×/9.77× | 9.15×/9.30× | 9.67×/9.82× |
| RollingConditionalValueAtRisk | Wickra `ConditionalValueAtRisk` | 2.52×/2.64× | 1.40×/1.42× | 1.29×/1.28× | 1.24×/1.27× |
| RollingCorrelation | TA-Lib `CORREL` | 3.75×/4.88× | 1.08×/1.10× | 1.06×/1.19× | 1.05×/1.15× |
| RollingCovariance | Wickra `RollingCovariance` | 14.20×/15.52× | 8.07×/8.29× | 7.44×/7.58× | 7.41×/7.60× |
| RollingDrawdownDuration | Wickra `DrawdownDuration` | 22.02×/25.46× | 16.34×/17.60× | 14.41×/16.01× | 15.69×/18.62× |
| RollingEntropy | NumPy `rolling Shannon entropy` | 0.07×/0.08× | 0.03×/0.03× | 0.01×/0.01× | 0.02×/0.02× |
| RollingGainLossRatio | Wickra `GainLossRatio` | 5.97×/5.82× | 2.27×/2.30× | 1.85×/1.85× | 2.05×/2.13× |
| RollingGrangerCausality | Wickra `GrangerCausality` | 4.39×/4.47× | 4.20×/4.26× | 4.06×/4.45× | 4.40×/4.25× |
| RollingInformationRatio | Wickra `InformationRatio` | 7.65×/7.93× | 2.58×/2.45× | 1.95×/2.11× | 2.11×/1.83× |
| RollingInterquartileRange | Wickra `RollingIqr` | 6.19×/6.17× | 2.49×/2.60× | 3.16×/2.97× | 2.24×/2.32× |
| RollingKellyCriterion | Wickra `KellyCriterion` | 8.21×/8.36× | 3.91×/3.87× | 3.68×/3.59× | 3.26×/3.36× |
| RollingKendallRankCorrelation | Wickra `KendallTau` | 1.77×/1.79× | 1.06×/1.13× | 1.10×/1.11× | 1.04×/1.04× |
| RollingKurtosis | Wickra `Kurtosis` | 8.18×/8.51× | 3.19×/3.24× | 2.62×/2.61× | 2.46×/2.40× |
| RollingLeadLagCrossCorrelation | Wickra `LeadLagCrossCorrelation` | 1.63×/1.63× | 1.39×/1.40× | 1.40×/1.24× | 1.45×/1.44× |
| RollingLinearRegression | TA-Lib `LINEARREG` | 2.73×/2.61× | 1.19×/1.17× | 1.00×/1.08× | 0.77×/1.01× |
| RollingLinearRegressionAngle | TA-Lib `LINEARREG_ANGLE` | 2.03×/2.14× | 1.16×/1.11× | 0.89×/0.99× | 0.95×/0.98× |
| RollingLinearRegressionIntercept | TA-Lib `LINEARREG_INTERCEPT` | 2.77×/2.69× | 1.24×/1.23× | 1.00×/0.95× | 1.04×/1.12× |
| RollingLinearRegressionSlope | TA-Lib `LINEARREG_SLOPE` | 2.79×/2.78× | 1.34×/1.38× | 0.98×/1.01× | 0.91×/0.96× |
| RollingMaximum | TA-Lib `MAX` | 5.76×/6.98× | 2.33×/2.45× | 1.44×/1.45× | 1.10×/1.22× |
| RollingMaximumDrawdown | Wickra `MaxDrawdown` | 5.07×/5.09× | 2.78×/2.83× | 2.60×/2.59× | 2.64×/2.65× |
| RollingMaximumIndex | TA-Lib `MAXINDEX` | 4.45×/5.18× | 1.61×/1.89× | 1.41×/1.31× | 1.04×/1.28× |
| RollingMedian | Wickra `MedianMA` | 8.73×/11.19× | 5.16×/5.20× | 5.86×/5.73× | 4.43×/4.58× |
| RollingMedianAbsoluteDeviation | Wickra `MedianAbsoluteDeviation` | 1.46×/1.47× | 1.29×/1.23× | 1.08×/1.12× | 1.08×/1.11× |
| RollingMidpoint | TA-Lib `MIDPOINT` | 4.39×/4.87× | 1.23×/1.27× | 0.84×/0.84× | 0.73×/0.77× |
| RollingMidprice | TA-Lib `MIDPRICE` | 4.28×/4.75× | 1.21×/1.28× | 0.98×/0.90× | 0.76×/0.83× |
| RollingMinMax | TA-Lib `MINMAX` | 4.36×/5.19× | 1.54×/1.64× | 1.39×/1.49× | 0.69×/0.98× |
| RollingMinMaxIndex | TA-Lib `MINMAXINDEX` | 1.96×/1.99× | 0.56×/0.57× | 0.43×/0.46× | 0.42×/0.43× |
| RollingMinimum | TA-Lib `MIN` | 6.06×/6.59× | 2.11×/2.30× | 1.38×/1.55× | 1.09×/1.26× |
| RollingMinimumIndex | TA-Lib `MININDEX` | 4.35×/5.82× | 1.79×/1.92× | 1.18×/1.20× | 1.16×/1.39× |
| RollingMode | NumPy `rolling mode` | 0.19×/0.20× | 0.05×/0.05× | 0.05×/0.04× | 0.06×/0.06× |
| RollingOmegaRatio | Wickra `OmegaRatio` | 6.29×/6.23× | 2.54×/2.60× | 1.88×/1.86× | 1.70×/1.87× |
| RollingPainIndex | Wickra `PainIndex` | 3.61×/3.51× | 1.51×/1.50× | 1.38×/1.37× | 1.24×/1.26× |
| RollingPairwiseBeta | Wickra `PairwiseBeta` | 6.55×/6.83× | 3.89×/3.93× | 3.17×/3.26× | 2.95×/3.18× |
| RollingPercentile | NumPy `rolling percentile` | 8.05×/8.08× | 4.86×/4.89× | 4.46×/4.49× | 4.50×/4.55× |
| RollingProfitFactor | Wickra `ProfitFactor` | 5.79×/6.25× | 2.84×/2.63× | 2.12×/2.30× | 2.14×/2.17× |
| RollingQuantile | Wickra `RollingQuantile` | 6.85×/7.37× | 3.55×/3.64× | 3.36×/3.38× | 3.30×/3.43× |
| RollingRank | NumPy `rolling percentile rank` | 6.62×/7.12× | 4.22×/4.23× | 3.83×/3.93× | 4.01×/4.05× |
| RollingRecoveryFactor | NumPy `rolling recovery factor on equity` | 3.51×/3.48× | 2.62×/2.63× | 3.03×/2.92× | 3.07×/2.92× |
| RollingSharpe | Wickra `SharpeRatio` | 5.41×/5.41× | 1.93×/1.89× | 1.45×/1.48× | 1.44×/1.07× |
| RollingSkew | Wickra `Skewness` | 5.12×/5.19× | 2.08×/2.11× | 1.74×/1.79× | 1.76×/1.75× |
| RollingSortino | Wickra `SortinoRatio` | 9.95×/10.32× | 3.75×/3.79× | 3.17×/3.16× | 3.08×/3.15× |
| RollingSpearmanCorrelation | Wickra `SpearmanCorrelation` | 1.89×/2.02× | 1.55×/1.61× | 1.62×/1.59× | 1.54×/1.56× |
| RollingStandardDeviation | TA-Lib `STDDEV` | 5.39×/5.74× | 1.33×/1.44× | 0.73×/0.78× | 0.69×/0.77× |
| RollingStandardError | Wickra `StandardError` | 5.85×/6.08× | 2.29×/2.32× | 1.91×/1.88× | 1.83×/1.83× |
| RollingSum | TA-Lib `SUM` | 5.88×/6.26× | 1.48×/1.63× | 0.64×/0.76× | 0.54×/0.63× |
| RollingTimeSeriesForecast | TA-Lib `TSF` | 2.57×/2.86× | 1.11×/1.19× | 0.89×/0.93× | 0.72×/1.01× |
| RollingTreynorRatio | Wickra `TreynorRatio` | 8.55×/8.90× | 4.36×/4.29× | 3.84×/3.93× | 3.59×/3.58× |
| RollingValueAtRisk | Wickra `ValueAtRisk` | 2.49×/2.35× | 1.43×/1.44× | 1.20×/1.18× | 1.28×/1.27× |
| RollingVariance | TA-Lib `VAR` | 5.32×/5.86× | 1.41×/1.49× | 0.70×/0.72× | 0.49×/0.65× |
| RollingVarianceRatio | Wickra `VarianceRatio` | 2.17×/2.16× | 1.23×/1.27× | 1.15×/1.14× | 1.28×/1.28× |
| RollingVolumeWeightedAveragePrice | Wickra `RollingVWAP` | 7.62×/8.92× | 6.10×/5.93× | 3.09×/4.81× | 6.44×/6.69× |
| RollingWinsorize | NumPy `rolling winsorize` | 10.11×/10.35× | 5.52×/5.51× | 5.88×/4.59× | 5.82×/5.99× |
| RollingZScore | Wickra `ZScore` | 6.12×/6.14× | 1.51×/2.10× | 1.30×/1.33× | 1.38×/1.40× |
| RoofingFilter | Wickra `RoofingFilter` | 25.42×/26.87× | 9.99×/10.43× | 7.87×/8.19× | 7.53×/8.12× |
| SchaffTrendCycle | pandas-ta-classic `stc` | 502.22×/526.63× | 452.20×/458.19× | 469.67×/475.04× | 436.87×/452.14× |
| SessionExtrema | NumPy `explicit-session extrema` | 46.54×/55.25× | 79.37×/91.41× | 81.83×/107.27× | 79.16×/88.64× |
| SessionRange | Wickra `SessionRange` | 24.92×/28.35× | 33.13×/35.87× | 31.10×/32.95× | 34.11×/38.13× |
| SessionVolumeLevels | NumPy `anchored volume levels` | 244.63×/255.96× | 291.74×/300.38× | 278.46×/290.61× | 286.47×/296.78× |
| SessionVolumeWeightedAveragePrice | Wickra `SessionVwap` | 11.37×/26.45× | 31.26×/33.74× | 33.74×/33.70× | 36.94×/37.29× |
| Sessions | SMC `smartmoneyconcepts.smc.sessions` | 5993.48×/7182.29× | 10282.17×/11123.61× | 10079.72×/11551.42× | 3698.55×/10753.04× |
| SharkPattern | Wickra `Shark` | 15.45×/18.04× | 14.08×/14.69× | 13.52×/13.65× | 12.37×/14.02× |
| SignalDelay | NumPy `signal delay` | 3.86×/4.49× | 0.77×/0.86× | 0.19×/0.21× | 0.27×/0.31× |
| SignedPower | NumPy `numpy.sign/abs/power` | 4.25×/6.04× | 1.99×/2.28× | 1.05×/1.18× | 1.39×/1.66× |
| SimpleMovingAverage | TA-Lib `SMA` | 6.96×/7.82× | 2.00×/2.30× | 0.93×/1.11× | 0.81×/0.93× |
| SmoothedTrendChannel | NumPy `smoothed trend channel` | 33.71×/39.54× | 41.43×/43.86× | 40.33×/42.23× | 39.58×/41.67× |
| SpreadZScore | NumPy `rolling hedged-spread z-score` | 4.69×/4.57× | 3.10×/3.03× | 4.07×/3.80× | 4.29×/4.13× |
| Squeeze | pandas-ta-classic `squeeze` | 102.14×/107.81× | 16.97×/17.38× | 7.66×/8.86× | 8.07×/8.33× |
| SqueezePro | pandas-ta-classic `squeeze_pro` | 105.68×/119.97× | 30.25×/31.65× | 14.35×/15.35× | 8.18×/14.22× |
| StandardErrorBands | Wickra `StandardErrorBands` | 8.46×/8.64× | 6.38×/6.47× | 5.94×/6.39× | 7.21×/7.35× |
| StochasticOscillator | TA-Lib `STOCH` | 3.42×/3.56× | 1.08×/1.15× | 0.90×/0.88× | 0.77×/0.82× |
| StochasticRelativeStrengthIndex | TA-Lib `STOCHRSI` | 2.15×/2.45× | 0.83×/0.87× | 0.61×/0.65× | 0.60×/0.63× |
| SuperSmoother | Wickra `SuperSmoother` | 17.72×/19.56× | 9.25×/9.37× | 6.28×/7.44× | 7.06×/7.78× |
| Supertrend | pandas-ta-classic `supertrend` | 74.47×/83.75× | 15.17×/15.49× | 7.69×/8.30× | 4.37×/8.92× |
| SwingHighLow | NumPy `causal confirmed swing pivots` | 80.28×/92.04× | 101.41×/101.92× | 107.49×/108.10× | 65.52×/98.21× |
| ThreeDrives | Wickra `ThreeDrives` | 15.65×/19.06× | 12.96×/14.06× | 14.40×/14.93× | 12.69×/13.34× |
| TimeOfDayReturnProfile | Wickra `TimeOfDayReturnProfile` | 31.01×/37.07× | 36.84×/37.55× | 37.66×/48.98× | 12.97×/23.24× |
| TimeSegmentedVolume | Wickra `TSV` | 20.84×/24.16× | 15.33×/15.88× | 13.93×/14.49× | 12.09×/13.90× |
| TimeSeriesRank | NumPy `rolling percentile rank` | 6.98×/7.26× | 4.38×/4.31× | 3.90×/3.98× | 3.99×/3.97× |
| TomDeMarkSequential | Wickra `TDSequential` | 67.09×/73.46× | 71.32×/76.21× | 59.60×/61.76× | 61.65×/71.08× |
| TradeVolumeIndex | Wickra `TradeVolumeIndex` | 26.17×/30.54× | 10.83×/11.41× | 9.74×/10.13× | 9.58×/10.43× |
| TrianglePattern | Wickra `Triangle` | 15.18×/19.17× | 13.06×/12.90× | 13.01×/13.27× | 12.84×/13.32× |
| TriangularMovingAverage | TA-Lib `TRIMA` | 5.02×/5.79× | 1.40×/1.48× | 0.81×/0.88× | 0.56×/0.71× |
| TripleExponentialAverage | TA-Lib `T3` | 4.76×/5.28× | 1.87×/1.92× | 1.26×/1.37× | 0.99×/1.16× |
| TripleExponentialMovingAverage | TA-Lib `TEMA` | 3.19×/3.14× | 0.96×/1.13× | 0.96×/0.90× | 0.95×/0.91× |
| TripleExponentialRateOfChange | TA-Lib `TRIX` | 6.81×/5.99× | 4.37×/4.08× | 3.86×/4.26× | 3.58×/4.52× |
| TripleTopBottom | Wickra `TripleTopBottom` | 19.21×/22.98× | 14.76×/15.93× | 14.33×/12.84× | 13.97×/12.72× |
| TrueRange | TA-Lib `TRANGE` | 2.70×/3.78× | 0.77×/0.91× | 0.26×/0.28× | 0.37×/0.47× |
| TrueStrengthIndex | Wickra `TSI` | 11.60×/12.36× | 4.33×/4.30× | 3.36×/3.31× | 3.05×/3.24× |
| TwiggsMoneyFlow | Wickra `TwiggsMoneyFlow` | 15.76×/18.59× | 17.00×/17.42× | 14.59×/15.38× | 11.89×/12.24× |
| TypicalPrice | TA-Lib `TYPPRICE` | 4.34×/5.60× | 1.54×/1.83× | 0.47×/0.58× | 0.59×/0.72× |
| UlcerIndex | Wickra `UlcerIndex` | 8.76×/8.87× | 2.51×/3.02× | 2.31×/2.45× | 2.42×/2.46× |
| UltimateOscillator | TA-Lib `ULTOSC` | 2.51×/2.83× | 1.30×/1.31× | 0.75×/1.14× | 1.08×/1.12× |
| UpDownVolumeRatio | Wickra `UpDownVolumeRatio` | 636.42×/787.92× | 1504.13×/1708.29× | 1911.70×/1580.87× | 1581.98×/1949.73× |
| ValueWhen | NumPy `last value when condition` | 25.39×/31.00× | 57.24×/64.82× | 73.03×/82.30× | 64.69×/79.43× |
| VariableIndexDynamicAverage | Wickra `VIDYA` | 14.62×/15.01× | 4.94×/5.01× | 3.61×/3.71× | 3.36×/3.54× |
| VariablePeriodMovingAverage | TA-Lib `MAVP` | 0.88×/0.94× | 0.64×/0.66× | 0.62×/0.70× | 0.81×/0.83× |
| VolumeByTimeProfile | Wickra `VolumeByTimeProfile` | 26.34×/30.26× | 27.49×/31.04× | 26.54×/35.69× | 11.75×/19.92× |
| VolumeOscillator | Wickra `VolumeOscillator` | 13.35×/9.88× | 4.52×/5.04× | 3.46×/3.89× | 3.37×/3.89× |
| VolumePriceTrend | Wickra `VolumePriceTrend` | 20.76×/25.78× | 21.35×/24.59× | 24.28×/26.47× | 19.67×/23.58× |
| VolumeRelativeStrengthIndex | Wickra `VolumeRsi` | 17.62×/18.83× | 7.74×/8.46× | 7.19×/7.41× | 8.29×/8.44× |
| VolumeWeightedMovingAverage | Wickra `VWMA` | 11.26×/12.33× | 6.34×/6.17× | 5.03×/4.94× | 4.96×/3.02× |
| VolumeWeightedMovingAverageConvergenceDivergence | Wickra `VolumeWeightedMacd` | 26.47×/28.90× | 23.92×/24.43× | 24.34×/25.91× | 25.21×/27.41× |
| VolumeZoneOscillator | Wickra `VZO` | 12.26×/13.58× | 6.64×/6.37× | 6.03×/5.56× | 5.99×/5.74× |
| Vortex | Wickra `Vortex` | 28.36×/32.76× | 33.42×/34.22× | 38.77×/39.47× | 39.82×/42.88× |
| WedgePattern | Wickra `Wedge` | 13.93×/21.90× | 15.68×/16.48× | 14.74×/15.29× | 14.25×/15.12× |
| WeightedClose | TA-Lib `WCLPRICE` | 4.13×/5.73× | 1.56×/1.97× | 0.52×/0.55× | 0.60×/0.73× |
| WeightedMovingAverage | TA-Lib `WMA` | 5.70×/5.74× | 1.30×/1.39× | 0.71×/0.68× | 0.52×/0.60× |
| WilliamsAccumulationDistribution | Wickra `Wad` | 17.94×/17.90× | 16.17×/17.07× | 14.82×/15.63× | 13.53×/16.94× |
| WilliamsPercentR | TA-Lib `WILLR` | 1.28×/1.51× | 0.44×/0.48× | 0.26×/0.32× | 0.30×/0.30× |
| YangZhang | Wickra `YangZhangVolatility` | 5.74×/6.44× | 4.18×/4.32× | 3.74×/3.74× | 3.59×/3.70× |
| ZeroLagExponentialMovingAverage | Wickra `ZLEMA` | 19.21×/22.22× | 19.52×/15.06× | 5.43×/4.81× | 8.09×/9.27× |
| ZigZag | Wickra `ZigZag` | 48.44×/45.13× | 41.90×/44.47× | 49.38×/52.45× | 51.68×/59.20× |

Complete vector and warm-up/thread tables plus raw samples are stored under `verify/evidence/benchmark/`.
