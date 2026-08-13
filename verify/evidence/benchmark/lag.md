# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.40M | 0.035 | 28.30M | 0.024 | 0.61× | 0.68× |
| 10,000 | 0.285 | 35.11M | 0.287 | 34.80M | 0.030 | 0.10× | 0.10× |
| 100,000 | 2.723 | 36.73M | 2.705 | 36.96M | 0.067 | 0.02× | 0.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.151 | 1.09× |
| 1 | 5 | 0.386 | 0.410 | 1.06× |
| 1 | 10 | 0.595 | 0.781 | 1.31× |
| 10 | 1 | 0.064 | 0.086 | 1.35× |
| 10 | 5 | 0.280 | 0.407 | 1.45× |
| 10 | 10 | 0.612 | 0.865 | 1.41× |
| 100 | 1 | 0.069 | 0.080 | 1.16× |
| 100 | 5 | 0.278 | 0.398 | 1.43× |
| 100 | 10 | 0.607 | 0.848 | 1.40× |
| 1,000 | 1 | 0.089 | 0.088 | 0.99× |
| 1,000 | 5 | 0.332 | 0.449 | 1.35× |
| 1,000 | 10 | 0.653 | 0.895 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
