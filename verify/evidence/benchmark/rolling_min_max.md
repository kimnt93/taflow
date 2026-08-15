# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.64M | 0.006 | 166.68M | 0.043 | 6.16× | 7.15× |
| 10,000 | 0.051 | 195.05M | 0.047 | 213.76M | 0.119 | 2.31× | 2.53× |
| 100,000 | 1.253 | 79.82M | 0.431 | 232.19M | 0.869 | 0.69× | 2.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.111 | 0.84× |
| 1 | 5 | 0.187 | 0.497 | 2.65× |
| 1 | 10 | 0.481 | 1.008 | 2.10× |
| 10 | 1 | 0.042 | 0.101 | 2.42× |
| 10 | 5 | 0.184 | 0.463 | 2.52× |
| 10 | 10 | 0.384 | 1.051 | 2.73× |
| 100 | 1 | 0.046 | 0.101 | 2.20× |
| 100 | 5 | 0.201 | 0.465 | 2.31× |
| 100 | 10 | 0.403 | 0.986 | 2.45× |
| 1,000 | 1 | 0.049 | 0.111 | 2.25× |
| 1,000 | 5 | 0.247 | 0.525 | 2.12× |
| 1,000 | 10 | 0.420 | 1.089 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
