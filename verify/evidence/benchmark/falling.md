# Falling benchmark (`period-over-period falling` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.97M | 0.054 | 18.43M | 0.030 | 0.55× | 0.56× |
| 10,000 | 0.449 | 22.28M | 0.429 | 23.33M | 0.039 | 0.09× | 0.09× |
| 100,000 | 4.319 | 23.16M | 4.321 | 23.14M | 0.130 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.104 | 0.96× |
| 1 | 5 | 0.394 | 0.458 | 1.16× |
| 1 | 10 | 0.590 | 0.907 | 1.54× |
| 10 | 1 | 0.063 | 0.090 | 1.43× |
| 10 | 5 | 0.293 | 0.433 | 1.47× |
| 10 | 10 | 0.582 | 0.935 | 1.61× |
| 100 | 1 | 0.067 | 0.096 | 1.45× |
| 100 | 5 | 0.304 | 0.447 | 1.47× |
| 100 | 10 | 0.621 | 0.932 | 1.50× |
| 1,000 | 1 | 0.110 | 0.091 | 0.82× |
| 1,000 | 5 | 0.301 | 0.483 | 1.60× |
| 1,000 | 10 | 0.668 | 1.152 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
