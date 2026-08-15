# RollingSharpe benchmark (`SharpeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.59M | 0.030 | 33.84M | 0.181 | 5.90× | 6.12× |
| 10,000 | 0.296 | 33.76M | 0.299 | 33.40M | 0.541 | 1.83× | 1.81× |
| 100,000 | 3.053 | 32.76M | 3.017 | 33.15M | 4.058 | 1.33× | 1.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.267 | 3.25× |
| 1 | 5 | 0.260 | 1.343 | 5.16× |
| 1 | 10 | 0.415 | 2.319 | 5.59× |
| 10 | 1 | 0.046 | 0.218 | 4.79× |
| 10 | 5 | 0.192 | 1.409 | 7.32× |
| 10 | 10 | 0.399 | 2.334 | 5.84× |
| 100 | 1 | 0.050 | 0.235 | 4.72× |
| 100 | 5 | 0.201 | 1.297 | 6.45× |
| 100 | 10 | 0.433 | 2.432 | 5.62× |
| 1,000 | 1 | 0.080 | 0.267 | 3.35× |
| 1,000 | 5 | 0.209 | 1.481 | 7.08× |
| 1,000 | 10 | 0.427 | 2.811 | 6.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
