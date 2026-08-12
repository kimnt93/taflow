# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.027 | 37.56M | 0.022 | 45.54M | 0.040 | 1.51× | 1.84× |
| 10,000 | 0.188 | 53.06M | 0.185 | 54.14M | 0.119 | 0.63× | 0.64× |
| 100,000 | 1.951 | 51.27M | 1.774 | 56.38M | 0.905 | 0.46× | 0.51× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.161 | 0.89× |
| 1 | 5 | 0.426 | 0.477 | 1.12× |
| 1 | 10 | 0.562 | 0.998 | 1.77× |
| 10 | 1 | 0.061 | 0.085 | 1.39× |
| 10 | 5 | 0.263 | 0.430 | 1.64× |
| 10 | 10 | 0.555 | 1.006 | 1.81× |
| 100 | 1 | 0.058 | 0.096 | 1.65× |
| 100 | 5 | 0.251 | 0.481 | 1.92× |
| 100 | 10 | 0.544 | 0.994 | 1.83× |
| 1,000 | 1 | 0.085 | 0.107 | 1.25× |
| 1,000 | 5 | 0.269 | 0.492 | 1.83× |
| 1,000 | 10 | 0.587 | 0.978 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
