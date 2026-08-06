use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use taflow::stream::{
    self, Accbands, Apo, Atr, Bbands, Dema, Ema, Imi, Ma, Macd, Mama, Midpoint, Midprice, Mom,
    Natr, Ppo, Roc, Rocp, Rocr, Rocr100, Rsi, Sar, Sarext, Sma, StreamingIndicator, Tema, Trange,
    Trima, Wma, T3,
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
            let mut state = Sma::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Ema::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("wma", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Wma::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("dema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Dema::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("tema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Tema::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("trima", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Trima::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("mama", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Mama::new(0.5, 0.05).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("t3", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = T3::new(20, 0.7).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("apo_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Apo::new(12, 26, MaType::Ema).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ppo_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Ppo::new(12, 26, MaType::Ema).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("ma_ema", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Ma::new(20, MaType::Ema).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("bbands_sma", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Bbands::new(20, 2.0, 2.0, MaType::Sma).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("accbands", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Accbands::new(20).unwrap();
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
            let mut state = Sar::default();
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
            let mut state = Sarext::default();
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
            let mut state = Midpoint::new(20).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("midprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Midprice::new(20).unwrap();
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
            let mut state = Mom::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("roc", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Roc::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocp", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Rocp::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Rocr::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rocr100", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Rocr100::new(10).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("rsi", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Rsi::new(14).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("atr", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Atr::new(14).unwrap();
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
            let mut state = Natr::new(14).unwrap();
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
            let mut state = Trange::new();
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
            let mut state = Macd::new(12, 26, 9).unwrap();
            for value in warmup {
                state.append(*value);
            }
            for value in updates {
                black_box(state.append(*value));
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
    bench_periodic!("max", Max);
    bench_periodic!("maxindex", Maxindex);
    bench_periodic!("min", Min);
    bench_periodic!("minindex", Minindex);
    bench_periodic!("sum", Sum);
    bench_periodic!("avgdev", Avgdev);
    bench_periodic!("cmo", Cmo);
    group.bench_function(BenchmarkId::new("imi", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = Imi::new(14).unwrap();
            for value in warmup {
                state.append(*value - 0.2, *value);
            }
            for value in updates {
                black_box(state.append(*value - 0.2, *value));
            }
        })
    });
    bench_periodic!("kama", Kama);
    bench_periodic!("linearreg", Linearreg);
    bench_periodic!("linearreg_slope", LinearregSlope);
    bench_periodic!("linearreg_intercept", LinearregIntercept);
    bench_periodic!("linearreg_angle", LinearregAngle);
    bench_periodic!("tsf", Tsf);
    group.bench_function(BenchmarkId::new("minmax", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::Minmax::new(20).unwrap();
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
            let mut state = stream::Minmaxindex::new(20).unwrap();
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
            let mut state = stream::Var::new(20, 1.0).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("stddev", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::Stddev::new(20, 2.0).unwrap();
            state.extend(warmup.iter().copied());
            for value in updates {
                black_box(state.append(*value));
            }
        })
    });
    group.bench_function(BenchmarkId::new("beta", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::Beta::new(20).unwrap();
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
            let mut state = stream::Correl::new(20).unwrap();
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
            let mut state = stream::Ad::new();
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
            let mut state = stream::Adosc::new(3, 10).unwrap();
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
            let mut state = stream::Obv::new();
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
            let mut state = stream::Bop::new();
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
            let mut state = stream::Willr::new(14).unwrap();
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
            let mut state = stream::Aroonosc::new(14).unwrap();
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
                    let mut state = stream::$state::new();
                    state.extend(warmup.iter().copied());
                    for value in updates {
                        black_box(state.append(*value));
                    }
                })
            });
        };
    }
    bench_unary!("acos", Acos);
    bench_unary!("asin", Asin);
    bench_unary!("atan", Atan);
    bench_unary!("ceil", Ceil);
    bench_unary!("cos", Cos);
    bench_unary!("cosh", Cosh);
    bench_unary!("exp", Exp);
    bench_unary!("floor", Floor);
    bench_unary!("ln", Ln);
    bench_unary!("log10", Log10);
    bench_unary!("sin", Sin);
    bench_unary!("sinh", Sinh);
    bench_unary!("sqrt", Sqrt);
    bench_unary!("tan", Tan);
    bench_unary!("tanh", Tanh);

    macro_rules! bench_binary {
        ($name:literal, $state:ident) => {
            group.bench_function(BenchmarkId::new($name, updates.len()), |bench| {
                bench.iter(|| {
                    let mut state = stream::$state::new();
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
    bench_binary!("add", Add);
    bench_binary!("sub", Sub);
    bench_binary!("mult", Mult);
    bench_binary!("div", Div);
    bench_binary!("medprice", Medprice);

    group.bench_function(BenchmarkId::new("avgprice", updates.len()), |bench| {
        bench.iter(|| {
            let mut state = stream::Avgprice::new();
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
            let mut state = stream::Typprice::new();
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
            let mut state = stream::Wclprice::new();
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
