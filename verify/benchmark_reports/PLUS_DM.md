# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.27M | 0.008 | 125.64M | 0.038 | 3.88× | 4.76× |
| 10,000 | 0.058 | 171.08M | 0.057 | 176.24M | 0.084 | 1.44× | 1.48× |
| 100,000 | 0.553 | 180.72M | 0.517 | 193.54M | 0.549 | 0.99× | 1.06× |
| 1,000,000 | 5.900 | 169.49M | 5.337 | 187.37M | 5.210 | 0.88× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.120 | 1.11× |
| 1 | 5 | 0.278 | 0.506 | 1.82× |
| 1 | 10 | 0.463 | 1.008 | 2.18× |
| 10 | 1 | 0.058 | 0.106 | 1.83× |
| 10 | 5 | 0.231 | 0.484 | 2.09× |
| 10 | 10 | 0.479 | 0.938 | 1.96× |
| 100 | 1 | 0.051 | 0.094 | 1.85× |
| 100 | 5 | 0.265 | 0.545 | 2.06× |
| 100 | 10 | 0.545 | 1.050 | 1.93× |
| 1,000 | 1 | 0.061 | 0.099 | 1.62× |
| 1,000 | 5 | 0.265 | 0.537 | 2.02× |
| 1,000 | 10 | 0.533 | 1.105 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
