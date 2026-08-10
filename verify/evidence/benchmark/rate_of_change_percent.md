# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 187.97M | 0.006 | 181.51M | 0.034 | 6.36× | 6.14× |
| 10,000 | 0.026 | 384.66M | 0.024 | 421.01M | 0.043 | 1.63× | 1.79× |
| 100,000 | 0.205 | 487.98M | 0.197 | 506.48M | 0.141 | 0.69× | 0.71× |
| 1,000,000 | 2.618 | 381.95M | 1.892 | 528.45M | 1.227 | 0.47× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.135 | 1.76× |
| 1 | 5 | 0.315 | 0.508 | 1.61× |
| 1 | 10 | 0.474 | 1.002 | 2.11× |
| 10 | 1 | 0.049 | 0.091 | 1.87× |
| 10 | 5 | 0.268 | 0.545 | 2.03× |
| 10 | 10 | 0.542 | 0.982 | 1.81× |
| 100 | 1 | 0.061 | 0.094 | 1.54× |
| 100 | 5 | 0.276 | 0.568 | 2.06× |
| 100 | 10 | 0.601 | 1.008 | 1.68× |
| 1,000 | 1 | 0.054 | 0.100 | 1.84× |
| 1,000 | 5 | 0.267 | 0.536 | 2.01× |
| 1,000 | 10 | 0.582 | 1.000 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
