# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.40M | 0.009 | 113.81M | 0.050 | 4.13× | 5.70× |
| 10,000 | 0.065 | 155.01M | 0.058 | 171.90M | 0.088 | 1.36× | 1.51× |
| 100,000 | 0.616 | 162.47M | 0.541 | 184.92M | 0.593 | 0.96× | 1.10× |
| 1,000,000 | 6.312 | 158.43M | 5.595 | 178.75M | 5.490 | 0.87× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.115 | 1.33× |
| 1 | 5 | 0.321 | 0.592 | 1.85× |
| 1 | 10 | 0.581 | 1.056 | 1.82× |
| 10 | 1 | 0.052 | 0.108 | 2.06× |
| 10 | 5 | 0.256 | 0.493 | 1.93× |
| 10 | 10 | 0.521 | 1.144 | 2.20× |
| 100 | 1 | 0.054 | 0.098 | 1.83× |
| 100 | 5 | 0.272 | 0.543 | 2.00× |
| 100 | 10 | 0.538 | 1.194 | 2.22× |
| 1,000 | 1 | 0.073 | 0.105 | 1.44× |
| 1,000 | 5 | 0.494 | 0.588 | 1.19× |
| 1,000 | 10 | 0.596 | 1.238 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
