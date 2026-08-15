# ChandeMomentumOscillator benchmark (`CMO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.47M | 0.006 | 169.22M | 0.038 | 5.55× | 6.46× |
| 10,000 | 0.057 | 175.39M | 0.053 | 187.55M | 0.090 | 1.58× | 1.69× |
| 100,000 | 0.553 | 180.87M | 0.533 | 187.45M | 0.596 | 1.08× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.107 | 0.66× |
| 1 | 5 | 0.218 | 0.494 | 2.26× |
| 1 | 10 | 0.406 | 0.973 | 2.40× |
| 10 | 1 | 0.042 | 0.089 | 2.13× |
| 10 | 5 | 0.182 | 0.497 | 2.73× |
| 10 | 10 | 0.387 | 0.957 | 2.47× |
| 100 | 1 | 0.039 | 0.094 | 2.40× |
| 100 | 5 | 0.174 | 0.449 | 2.58× |
| 100 | 10 | 0.407 | 1.035 | 2.55× |
| 1,000 | 1 | 0.059 | 0.102 | 1.73× |
| 1,000 | 5 | 0.210 | 0.472 | 2.24× |
| 1,000 | 10 | 0.400 | 1.033 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
