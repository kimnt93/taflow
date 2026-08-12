# CumulativeMaximum benchmark (`numpy.maximum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.72M | 0.005 | 185.21M | 0.016 | 2.69× | 3.04× |
| 10,000 | 0.035 | 285.18M | 0.030 | 337.32M | 0.042 | 1.20× | 1.42× |
| 100,000 | 0.319 | 313.12M | 0.297 | 336.69M | 0.296 | 0.93× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.128 | 1.12× |
| 1 | 5 | 0.291 | 0.307 | 1.05× |
| 1 | 10 | 0.467 | 0.615 | 1.32× |
| 10 | 1 | 0.048 | 0.061 | 1.26× |
| 10 | 5 | 0.221 | 0.273 | 1.23× |
| 10 | 10 | 0.495 | 0.608 | 1.23× |
| 100 | 1 | 0.052 | 0.072 | 1.38× |
| 100 | 5 | 0.227 | 0.273 | 1.21× |
| 100 | 10 | 0.475 | 0.613 | 1.29× |
| 1,000 | 1 | 0.062 | 0.078 | 1.26× |
| 1,000 | 5 | 0.256 | 0.314 | 1.22× |
| 1,000 | 10 | 0.491 | 0.707 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
