# SessionExtrema benchmark (`explicit-session extrema` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.41M | 0.052 | 19.13M | 0.511 | 8.39× | 9.78× |
| 10,000 | 0.431 | 23.22M | 0.420 | 23.78M | 5.085 | 11.81× | 12.09× |
| 100,000 | 4.497 | 22.24M | 4.072 | 24.56M | 51.559 | 11.46× | 12.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.103 | 0.76× |
| 1 | 5 | 0.333 | 0.307 | 0.92× |
| 1 | 10 | 0.604 | 0.623 | 1.03× |
| 10 | 1 | 0.067 | 0.071 | 1.05× |
| 10 | 5 | 0.290 | 0.327 | 1.13× |
| 10 | 10 | 0.629 | 0.683 | 1.09× |
| 100 | 1 | 0.070 | 0.124 | 1.77× |
| 100 | 5 | 0.303 | 0.545 | 1.80× |
| 100 | 10 | 0.594 | 1.153 | 1.94× |
| 1,000 | 1 | 0.119 | 0.573 | 4.82× |
| 1,000 | 5 | 0.319 | 2.860 | 8.98× |
| 1,000 | 10 | 0.652 | 5.785 | 8.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
