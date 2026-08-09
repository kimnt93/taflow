# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.70M | 0.004 | 230.09M | 0.030 | 3.42× | 6.80× |
| 10,000 | 0.012 | 800.17M | 0.009 | 1.12G | 0.034 | 2.69× | 3.77× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.004 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.397 | 0.252 | 3.97M | 29.851 | 118.63× | 109.14× |
| 1,500 | 10 | 2.052 | 0.985 | 10.15M | 28.531 | 28.96× | 28.20× |
| 1,500 | 100 | 4.021 | 2.274 | 43.98M | 29.569 | 13.01× | 11.95× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.95M | 11.69M | 1.00× | 907.70K | 1.36M | 1.00× | 9.78M |
| 2 | 16.21M | 20.24M | 1.73× | 1.31M | 1.55M | 1.14× | 11.00M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
