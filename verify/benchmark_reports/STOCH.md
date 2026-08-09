# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.56M | 0.014 | 73.05M | 0.050 | 3.41× | 3.69× |
| 10,000 | 0.139 | 72.06M | 0.136 | 73.29M | 0.161 | 1.16× | 1.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.020 ms**; TA-Lib 0.055 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.380 | 0.302 | 3.31M | 54.177 | 179.24× | 138.42× |
| 1,500 | 10 | 2.704 | 1.388 | 7.21M | 54.138 | 39.01× | 31.26× |
| 1,500 | 100 | 6.855 | 5.336 | 18.74M | 54.872 | 10.28× | 8.11× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.96M | 10.58M | 1.00× | 1.10M | 770.37K | 1.00× | 6.88M |
| 2 | 16.40M | 12.32M | 1.16× | 1.26M | 1.17M | 1.52× | 7.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
