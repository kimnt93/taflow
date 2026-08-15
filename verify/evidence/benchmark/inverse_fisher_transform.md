# InverseFisherTransform benchmark (`InverseFisherTransform` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 186.66M | 0.004 | 231.93M | 0.171 | 31.86× | 39.59× |
| 10,000 | 0.036 | 281.24M | 0.033 | 304.67M | 0.493 | 13.86× | 15.01× |
| 100,000 | 0.323 | 309.39M | 0.320 | 312.51M | 3.561 | 11.02× | 11.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.338 | 5.01× |
| 1 | 5 | 0.312 | 1.211 | 3.88× |
| 1 | 10 | 0.394 | 2.268 | 5.76× |
| 10 | 1 | 0.048 | 0.202 | 4.24× |
| 10 | 5 | 0.187 | 1.231 | 6.57× |
| 10 | 10 | 0.404 | 2.263 | 5.60× |
| 100 | 1 | 0.052 | 0.208 | 4.02× |
| 100 | 5 | 0.187 | 1.287 | 6.87× |
| 100 | 10 | 0.449 | 2.287 | 5.09× |
| 1,000 | 1 | 0.048 | 0.237 | 4.92× |
| 1,000 | 5 | 0.191 | 1.418 | 7.43× |
| 1,000 | 10 | 0.441 | 2.587 | 5.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
