# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.77M | 0.009 | 110.40M | 0.034 | 2.85× | 3.71× |
| 10,000 | 0.099 | 101.35M | 0.095 | 105.50M | 0.090 | 0.91× | 0.95× |
| 100,000 | 0.993 | 100.70M | 0.941 | 106.32M | 0.652 | 0.66× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.126 | 1.43× |
| 1 | 5 | 0.215 | 0.442 | 2.06× |
| 1 | 10 | 0.372 | 0.902 | 2.42× |
| 10 | 1 | 0.047 | 0.099 | 2.10× |
| 10 | 5 | 0.203 | 0.461 | 2.27× |
| 10 | 10 | 0.412 | 0.929 | 2.25× |
| 100 | 1 | 0.045 | 0.087 | 1.91× |
| 100 | 5 | 0.183 | 0.434 | 2.36× |
| 100 | 10 | 0.430 | 0.934 | 2.17× |
| 1,000 | 1 | 0.053 | 0.096 | 1.82× |
| 1,000 | 5 | 0.190 | 0.463 | 2.44× |
| 1,000 | 10 | 0.419 | 1.043 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
