# Crossunder benchmark (`causal crossunder` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.91M | 0.006 | 158.39M | 0.016 | 2.18× | 2.59× |
| 10,000 | 0.036 | 279.38M | 0.032 | 310.72M | 0.028 | 0.78× | 0.86× |
| 100,000 | 0.306 | 326.97M | 0.281 | 356.05M | 0.169 | 0.55× | 0.60× |
| 1,000,000 | 3.739 | 267.45M | 2.997 | 333.65M | 2.622 | 0.70× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.098 | 1.19× |
| 1 | 5 | 0.290 | 0.321 | 1.11× |
| 1 | 10 | 0.481 | 0.653 | 1.36× |
| 10 | 1 | 0.051 | 0.064 | 1.26× |
| 10 | 5 | 0.213 | 0.317 | 1.49× |
| 10 | 10 | 0.482 | 0.706 | 1.47× |
| 100 | 1 | 0.048 | 0.065 | 1.35× |
| 100 | 5 | 0.239 | 0.313 | 1.31× |
| 100 | 10 | 0.514 | 0.693 | 1.35× |
| 1,000 | 1 | 0.055 | 0.067 | 1.22× |
| 1,000 | 5 | 0.242 | 0.385 | 1.59× |
| 1,000 | 10 | 0.517 | 0.904 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
