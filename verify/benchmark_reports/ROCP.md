# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.42M | 0.005 | 215.92M | 0.037 | 5.07× | 7.91× |
| 10,000 | 0.026 | 383.23M | 0.024 | 425.39M | 0.042 | 1.61× | 1.78× |
| 100,000 | 0.222 | 450.26M | 0.199 | 502.03M | 0.138 | 0.62× | 0.69× |
| 1,000,000 | 2.715 | 368.30M | 2.386 | 419.03M | 1.273 | 0.47× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.114 | 1.49× |
| 1 | 5 | 0.330 | 0.505 | 1.53× |
| 1 | 10 | 0.532 | 1.001 | 1.88× |
| 10 | 1 | 0.051 | 0.091 | 1.80× |
| 10 | 5 | 0.248 | 0.495 | 1.99× |
| 10 | 10 | 0.541 | 1.012 | 1.87× |
| 100 | 1 | 0.049 | 0.094 | 1.92× |
| 100 | 5 | 0.235 | 0.459 | 1.95× |
| 100 | 10 | 0.551 | 1.064 | 1.93× |
| 1,000 | 1 | 0.058 | 0.097 | 1.68× |
| 1,000 | 5 | 0.246 | 0.456 | 1.85× |
| 1,000 | 10 | 0.547 | 1.040 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
