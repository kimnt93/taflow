# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.79M | 0.003 | 388.37M | 0.035 | 6.09× | 13.53× |
| 10,000 | 0.030 | 334.59M | 0.025 | 398.63M | 0.086 | 2.89× | 3.44× |
| 100,000 | 0.440 | 227.33M | 0.464 | 215.56M | 0.658 | 1.50× | 1.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.126 | 0.90× |
| 1 | 5 | 0.240 | 0.475 | 1.98× |
| 1 | 10 | 0.399 | 0.916 | 2.30× |
| 10 | 1 | 0.041 | 0.096 | 2.36× |
| 10 | 5 | 0.176 | 0.407 | 2.32× |
| 10 | 10 | 0.410 | 0.912 | 2.22× |
| 100 | 1 | 0.044 | 0.091 | 2.06× |
| 100 | 5 | 0.193 | 0.440 | 2.28× |
| 100 | 10 | 0.403 | 0.872 | 2.16× |
| 1,000 | 1 | 0.045 | 0.095 | 2.12× |
| 1,000 | 5 | 0.245 | 0.597 | 2.44× |
| 1,000 | 10 | 0.457 | 0.967 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
