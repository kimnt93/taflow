# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.89M | 0.004 | 258.93M | 0.031 | 4.42× | 8.13× |
| 10,000 | 0.072 | 139.09M | 0.063 | 157.83M | 0.089 | 1.24× | 1.41× |
| 100,000 | 0.869 | 115.11M | 0.869 | 115.06M | 0.645 | 0.74× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.113 | 1.83× |
| 1 | 5 | 0.305 | 0.449 | 1.47× |
| 1 | 10 | 0.372 | 0.869 | 2.34× |
| 10 | 1 | 0.039 | 0.095 | 2.43× |
| 10 | 5 | 0.199 | 0.434 | 2.18× |
| 10 | 10 | 0.381 | 0.906 | 2.38× |
| 100 | 1 | 0.041 | 0.090 | 2.20× |
| 100 | 5 | 0.192 | 0.423 | 2.20× |
| 100 | 10 | 0.421 | 0.924 | 2.19× |
| 1,000 | 1 | 0.048 | 0.103 | 2.13× |
| 1,000 | 5 | 0.187 | 0.471 | 2.52× |
| 1,000 | 10 | 0.436 | 1.019 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
