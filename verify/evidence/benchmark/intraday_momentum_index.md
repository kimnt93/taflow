# IntradayMomentumIndex benchmark (`IMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.63M | 0.006 | 158.15M | 0.087 | 11.25× | 13.73× |
| 10,000 | 0.056 | 177.21M | 0.054 | 186.46M | 0.648 | 11.49× | 12.08× |
| 100,000 | 0.547 | 182.73M | 0.522 | 191.39M | 6.174 | 11.28× | 11.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.117 | 2.07× |
| 1 | 5 | 0.302 | 0.498 | 1.65× |
| 1 | 10 | 0.392 | 1.061 | 2.71× |
| 10 | 1 | 0.052 | 0.101 | 1.93× |
| 10 | 5 | 0.197 | 0.553 | 2.81× |
| 10 | 10 | 0.411 | 0.945 | 2.30× |
| 100 | 1 | 0.051 | 0.096 | 1.90× |
| 100 | 5 | 0.193 | 0.503 | 2.60× |
| 100 | 10 | 0.403 | 0.998 | 2.48× |
| 1,000 | 1 | 0.048 | 0.159 | 3.30× |
| 1,000 | 5 | 0.203 | 0.746 | 3.67× |
| 1,000 | 10 | 0.481 | 1.568 | 3.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
