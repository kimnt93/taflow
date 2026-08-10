# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.99M | 0.022 | 45.46M | 0.050 | 1.95× | 2.27× |
| 10,000 | 0.192 | 52.07M | 0.185 | 53.98M | 0.227 | 1.18× | 1.23× |
| 100,000 | 1.964 | 50.92M | 1.951 | 51.25M | 2.011 | 1.02× | 1.03× |
| 1,000,000 | 19.278 | 51.87M | 19.245 | 51.96M | 20.276 | 1.05× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.125 | 1.02× |
| 1 | 5 | 0.301 | 0.446 | 1.48× |
| 1 | 10 | 0.614 | 0.925 | 1.51× |
| 10 | 1 | 0.057 | 0.099 | 1.75× |
| 10 | 5 | 0.251 | 0.435 | 1.73× |
| 10 | 10 | 0.524 | 1.082 | 2.07× |
| 100 | 1 | 0.063 | 0.088 | 1.39× |
| 100 | 5 | 0.280 | 0.431 | 1.54× |
| 100 | 10 | 0.524 | 0.963 | 1.84× |
| 1,000 | 1 | 0.087 | 0.117 | 1.36× |
| 1,000 | 5 | 0.330 | 0.611 | 1.85× |
| 1,000 | 10 | 0.575 | 1.121 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
