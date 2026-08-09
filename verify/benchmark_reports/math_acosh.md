# MathAcosh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.04M | 0.013 | 77.01M | 0.011 | 0.77× | 0.87× |
| 10,000 | 0.120 | 83.65M | 0.113 | 88.18M | 0.101 | 0.84× | 0.89× |
| 100,000 | 1.067 | 93.69M | 1.122 | 89.15M | 0.951 | 0.89× | 0.85× |
| 1,000,000 | 13.547 | 73.82M | 12.213 | 81.88M | 10.388 | 0.77× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.086 | 1.05× |
| 1 | 5 | 0.279 | 0.232 | 0.83× |
| 1 | 10 | 0.599 | 0.476 | 0.80× |
| 10 | 1 | 0.048 | 0.050 | 1.03× |
| 10 | 5 | 0.261 | 0.217 | 0.83× |
| 10 | 10 | 0.619 | 0.514 | 0.83× |
| 100 | 1 | 0.062 | 0.048 | 0.78× |
| 100 | 5 | 0.275 | 0.267 | 0.97× |
| 100 | 10 | 0.584 | 0.487 | 0.83× |
| 1,000 | 1 | 0.074 | 0.067 | 0.91× |
| 1,000 | 5 | 0.355 | 0.294 | 0.83× |
| 1,000 | 10 | 0.626 | 0.529 | 0.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
