# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.66M | 0.006 | 173.64M | 0.031 | 4.44× | 5.33× |
| 10,000 | 0.033 | 305.45M | 0.031 | 321.80M | 0.033 | 1.02× | 1.07× |
| 100,000 | 0.308 | 324.92M | 0.278 | 359.58M | 0.087 | 0.28× | 0.31× |
| 1,000,000 | 3.529 | 283.36M | 2.886 | 346.45M | 1.197 | 0.34× | 0.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.101 | 0.62× |
| 1 | 5 | 0.284 | 0.438 | 1.54× |
| 1 | 10 | 0.527 | 1.014 | 1.93× |
| 10 | 1 | 0.058 | 0.106 | 1.84× |
| 10 | 5 | 0.241 | 0.456 | 1.89× |
| 10 | 10 | 0.500 | 0.961 | 1.92× |
| 100 | 1 | 0.056 | 0.090 | 1.61× |
| 100 | 5 | 0.274 | 0.485 | 1.77× |
| 100 | 10 | 0.473 | 1.009 | 2.13× |
| 1,000 | 1 | 0.048 | 0.102 | 2.11× |
| 1,000 | 5 | 0.272 | 0.575 | 2.12× |
| 1,000 | 10 | 0.531 | 1.018 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
