use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use taflow::indicators::{
    AccelerationBands, MesaAdaptiveMovingAverage, Momentum, ParabolicSar, ParabolicSarExtended,
    RateOfChange, RateOfChangePercent, RateOfChangeRatio, RateOfChangeRatioPercent, RollingMinMax,
    RollingMinMaxIndex, RollingStandardDeviation, RollingVariance, WilliamsPercentR,
};
use taflow::stream::{
    AbsolutePriceOscillator, AverageTrueRange, BollingerBands, DoubleExponentialMovingAverage,
    ExponentialMovingAverage, FastStochasticOscillator, IntradayMomentumIndex, MovingAverage,
    MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceFixed,
    NormalizedAverageTrueRange, PercentagePriceOscillator, RelativeStrengthIndex,
    SimpleMovingAverage, StochasticOscillator, StreamingIndicator, TriangularMovingAverage,
    TripleExponentialAverage, TripleExponentialMovingAverage, TrueRange, WeightedMovingAverage,
};
use taflow::MaType;

fn prices(length: usize) -> Vec<f64> {
    (0..length)
        .map(|index| 100.0 + index as f64 * 0.001 + (index as f64 * 0.017).sin() * 4.0)
        .collect()
}

fn append_benchmark(criterion: &mut Criterion) {
    let data = prices(1_010_000);
    let warmup = &data[..10_000];
    let updates = &data[10_000..];
    let mut group = criterion.benchmark_group("stream_append_after_10k_warmup");

    group.bench_function(BenchmarkId::new("sma", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = SimpleMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = ExponentialMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("wma", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = WeightedMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("dema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = DoubleExponentialMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("tema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = TripleExponentialMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("trima", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = TriangularMovingAverage::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("mama", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = MesaAdaptiveMovingAverage::new(0.5, 0.05).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("t3", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = TripleExponentialAverage::new(20, 0.7).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("apo_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state =
                AbsolutePriceOscillator::new(12, 26, MaType::ExponentialMovingAverage).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ppo_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state =
                PercentagePriceOscillator::new(12, 26, MaType::ExponentialMovingAverage).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ma_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = MovingAverage::new(20, MaType::ExponentialMovingAverage).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("bbands_sma", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = BollingerBands::new(20, 2.0, 2.0, MaType::SimpleMovingAverage).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("accbands", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = AccelerationBands::new(20).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("sar", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = ParabolicSar::default();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("sarext", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = ParabolicSarExtended::default();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("midpoint", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RollingMidpoint::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("midprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RollingMidprice::new(20).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("mom", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Momentum::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("roc", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RateOfChange::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocp", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RateOfChangePercent::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RateOfChangeRatio::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocr100", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RateOfChangeRatioPercent::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rsi", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RelativeStrengthIndex::new(14).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("atr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = AverageTrueRange::new(14).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("natr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = NormalizedAverageTrueRange::new(14).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("trange", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = TrueRange::new().unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("macd", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
            for value in warmup {
                state.append(*value);
            }
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("macdfix", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = MovingAverageConvergenceDivergenceFixed::new(9).unwrap();
            for value in warmup {
                state.append(*value);
            }
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("stochf", updates.len()), |bench| {
        bench.iter(|| {
            let mut state =
                FastStochasticOscillator::new(5, 13, MaType::SimpleMovingAverage).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("stoch", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = StochasticOscillator::new(
                5,
                13,
                MaType::SimpleMovingAverage,
                11,
                MaType::SimpleMovingAverage,
            )
            .unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });

    macro_rules! bench_periodic {
        ($name:literal, $state:ident) => {
            group.bench_function(BenchmarkId::new($name, updates.len()), |bench| {
                bench.iter(|| {
                    let mut state = stream::$state::new(20).unwrap();
                    state.extend(warmup.iter().copied());
                    for value in updates {
                        black_box(state.append(*value));
                    }
                })
            });
        };
    }
    bench_periodic!("max", RollingMax);
    bench_periodic!("maxindex", RollingArgmax);
    bench_periodic!("min", RollingMin);
    bench_periodic!("minindex", RollingArgmin);
    bench_periodic!("sum", RollingSum);
    bench_periodic!("avgdev", RollingAverageDeviation);
    bench_periodic!("cmo", ChandeMomentumOscillator);
    group.bench_function(BenchmarkId::new("imi", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = IntradayMomentumIndex::new(14).unwrap();
            for value in warmup {
                state.append(*value - 0.2, *value);
            }
            for value in updates {
                black_box(state.append(*value - 0.2, *value));
            }
        })
    });
    bench_periodic!("kama", KaufmanAdaptiveMovingAverage);
    bench_periodic!("linearreg", RollingLinearRegression);
    bench_periodic!("linearreg_slope", LinearregSlope);
    bench_periodic!("linearreg_intercept", LinearregIntercept);
    bench_periodic!("linearreg_angle", LinearregAngle);
    bench_periodic!("tsf", RollingTimeSeriesForecast);
    group.bench_function(BenchmarkId::new("minmax", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RollingMinMax::new(20).unwrap();
            for value in warmup {
                state.append(*value);
            }
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("minmaxindex", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = RollingMinMaxIndex::new(20).unwrap();
            for value in warmup {
                state.append(*value);
            }
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("var", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = indicators::RollingVariance::new(20, 1.0).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("stddev", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = indicators::RollingStandardDeviation::new(20, 2.0).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("beta", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::RollingBeta::new(20).unwrap();
            for value in warmup {
                state.append(*value, *value * 1.2 + 0.5);
            }
            for value in updates {
                black_box(state.append(*value, *value * 1.2 + 0.5));
            }
        })
    });
    group.bench_function(BenchmarkId::new("correl", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::RollingCorrelation::new(20).unwrap();
            for value in warmup {
                state.append(*value, *value * 1.2 + 0.5);
            }
            for value in updates {
                black_box(state.append(*value, *value * 1.2 + 0.5));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ad", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::AccumulationDistribution::new().unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value + 0.2, 1_000.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value + 0.2, 1_000.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("adosc", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::AccumulationDistributionOscillator::new(3, 10).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value + 0.2, 1_000.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value + 0.2, 1_000.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("obv", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::OnBalanceVolume::new().unwrap();
            for value in warmup {
                state.append(*value, 1_000.0);
            }
            for value in updates {
                black_box(state.append(*value, 1_000.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("bop", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::BalanceOfPower::new().unwrap();
            for value in warmup {
                state.append(*value - 0.1, *value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value - 0.1, *value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("willr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = WilliamsPercentR::new(14).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("aroon", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::Aroon::new(14).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0));
            }
        })
    });
    group.bench_function(BenchmarkId::new("aroonosc", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::AroonOscillator::new(14).unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0));
            }
        })
    });

    macro_rules! bench_unary {
        ($name:literal, $state:ident) => {
            group.bench_function(BenchmarkId::new($name, updates.len()), |bench| {
                bench.iter(|| {
                    let mut state = stream::$state::new().unwrap();
                    state.extend(warmup.iter().copied());
                    for value in updates {
                        black_box(state.append(*value));
                    }
                })
            });
        };
    }
    bench_unary!("abs", MathAbs);
    bench_unary!("acos", MathAcos);
    bench_unary!("acosh", MathAcosh);
    bench_unary!("asin", MathAsin);
    bench_unary!("asinh", MathAsinh);
    bench_unary!("atan", MathAtan);
    bench_unary!("atanh", MathAtanh);
    bench_unary!("cbrt", MathCbrt);
    bench_unary!("ceil", MathCeil);
    bench_unary!("cos", MathCos);
    bench_unary!("cosh", MathCosh);
    bench_unary!("cot", MathCot);
    bench_unary!("degrees", MathDegrees);
    bench_unary!("exp", MathExp);
    bench_unary!("floor", MathFloor);
    bench_unary!("ln", MathLn);
    bench_unary!("log10", MathLog10);
    bench_unary!("log1p", MathLog1p);
    bench_unary!("radians", MathRadians);
    bench_unary!("sin", MathSin);
    bench_unary!("sinh", MathSinh);
    bench_unary!("sqrt", MathSqrt);
    bench_unary!("tan", MathTan);
    bench_unary!("tanh", MathTanh);

    macro_rules! bench_binary {
        ($name:literal, $state:ident) => {
            group.bench_function(BenchmarkId::new($name, updates.len()), |bench| {
                bench.iter(|| {
                    let mut state = stream::$state::new().unwrap();
                    for value in warmup {
                        state.append(*value, *value + 1.0);
                    }
                    for value in updates {
                        black_box(state.append(*value, *value + 1.0));
                    }
                })
            });
        };
    }
    bench_binary!("add", MathAdd);
    bench_binary!("sub", MathSubtract);
    bench_binary!("mult", MathMultiply);
    bench_binary!("div", MathDivide);
    bench_binary!("medprice", MedianPrice);

    group.bench_function(BenchmarkId::new("avgprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::AveragePrice::new().unwrap();
            for value in warmup {
                state.append(*value, *value + 1.0, *value - 1.0, *value + 0.1);
            }
            for value in updates {
                black_box(state.append(*value, *value + 1.0, *value - 1.0, *value + 0.1));
            }
        })
    });
    group.bench_function(BenchmarkId::new("typprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::TypicalPrice::new().unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("wclprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::WeightedClose::new().unwrap();
            for value in warmup {
                state.append(*value + 1.0, *value - 1.0, *value);
            }
            for value in updates {
                black_box(state.append(*value + 1.0, *value - 1.0, *value));
            }
        })
    });
    group.finish();
}

criterion_group!(benches, append_benchmark);
criterion_main!(benches);
