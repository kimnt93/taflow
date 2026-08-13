# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.091 | 11.02M | 0.081 | 12.38M | 0.043 | 0.47× | 0.53× |
| 10,000 | 0.629 | 15.89M | 0.631 | 15.86M | 0.161 | 0.26× | 0.26× |
| 100,000 | 6.021 | 16.61M | 6.207 | 16.11M | 1.491 | 0.25× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.142 | 1.09× |
| 1 | 5 | 0.490 | 0.493 | 1.01× |
| 1 | 10 | 0.660 | 0.908 | 1.38× |
| 10 | 1 | 0.067 | 0.084 | 1.26× |
| 10 | 5 | 0.306 | 0.406 | 1.33× |
| 10 | 10 | 0.653 | 0.934 | 1.43× |
| 100 | 1 | 0.083 | 0.092 | 1.11× |
| 100 | 5 | 0.305 | 0.425 | 1.39× |
| 100 | 10 | 0.658 | 0.905 | 1.37× |
| 1,000 | 1 | 0.139 | 0.103 | 0.74× |
| 1,000 | 5 | 0.316 | 0.519 | 1.64× |
| 1,000 | 10 | 0.730 | 1.048 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
