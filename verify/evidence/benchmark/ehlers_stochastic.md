# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.370 | 2.71M | 0.370 | 2.70M | 0.175 | 0.47× | 0.47× |
| 10,000 | 3.424 | 2.92M | 3.384 | 2.96M | 0.745 | 0.22× | 0.22× |
| 100,000 | 33.461 | 2.99M | 36.325 | 2.75M | 6.410 | 0.19× | 0.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.201 | 1.35× |
| 1 | 5 | 0.449 | 1.030 | 2.30× |
| 1 | 10 | 0.691 | 2.251 | 3.26× |
| 10 | 1 | 0.080 | 0.211 | 2.63× |
| 10 | 5 | 0.328 | 0.961 | 2.93× |
| 10 | 10 | 0.657 | 2.082 | 3.17× |
| 100 | 1 | 0.102 | 0.196 | 1.93× |
| 100 | 5 | 0.321 | 0.992 | 3.09× |
| 100 | 10 | 0.725 | 2.354 | 3.25× |
| 1,000 | 1 | 0.444 | 0.261 | 0.59× |
| 1,000 | 5 | 0.834 | 1.375 | 1.65× |
| 1,000 | 10 | 1.128 | 2.939 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
