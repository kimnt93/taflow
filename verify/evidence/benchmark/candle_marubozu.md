# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.18M | 0.017 | 58.03M | 0.037 | 1.74× | 2.14× |
| 10,000 | 0.150 | 66.79M | 0.146 | 68.38M | 0.159 | 1.06× | 1.09× |
| 100,000 | 1.405 | 71.15M | 1.462 | 68.39M | 1.101 | 0.78× | 0.75× |
| 1,000,000 | 14.706 | 68.00M | 14.427 | 69.32M | 10.901 | 0.74× | 0.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.104 | 0.57× |
| 1 | 5 | 0.478 | 0.482 | 1.01× |
| 1 | 10 | 0.566 | 0.914 | 1.62× |
| 10 | 1 | 0.056 | 0.085 | 1.52× |
| 10 | 5 | 0.250 | 0.463 | 1.85× |
| 10 | 10 | 0.607 | 0.976 | 1.61× |
| 100 | 1 | 0.056 | 0.099 | 1.77× |
| 100 | 5 | 0.292 | 0.534 | 1.83× |
| 100 | 10 | 0.646 | 1.033 | 1.60× |
| 1,000 | 1 | 0.079 | 0.106 | 1.34× |
| 1,000 | 5 | 0.270 | 0.490 | 1.81× |
| 1,000 | 10 | 0.565 | 1.153 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
