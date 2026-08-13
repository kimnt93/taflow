# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.76M | 0.062 | 16.24M | 0.034 | 0.47× | 0.55× |
| 10,000 | 0.552 | 18.10M | 0.516 | 19.38M | 0.152 | 0.28× | 0.29× |
| 100,000 | 5.219 | 19.16M | 5.180 | 19.30M | 1.375 | 0.26× | 0.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.122 | 0.99× |
| 1 | 5 | 0.364 | 0.443 | 1.22× |
| 1 | 10 | 0.635 | 0.924 | 1.46× |
| 10 | 1 | 0.066 | 0.089 | 1.34× |
| 10 | 5 | 0.315 | 0.433 | 1.38× |
| 10 | 10 | 0.633 | 0.896 | 1.42× |
| 100 | 1 | 0.072 | 0.100 | 1.38× |
| 100 | 5 | 0.308 | 0.429 | 1.39× |
| 100 | 10 | 0.660 | 0.921 | 1.39× |
| 1,000 | 1 | 0.126 | 0.098 | 0.78× |
| 1,000 | 5 | 0.314 | 0.497 | 1.58× |
| 1,000 | 10 | 0.681 | 1.062 | 1.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
