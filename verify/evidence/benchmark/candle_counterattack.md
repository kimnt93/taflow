# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.136 | 7.33M | 0.121 | 8.27M | 0.034 | 0.25× | 0.28× |
| 10,000 | 1.128 | 8.87M | 1.126 | 8.88M | 0.131 | 0.12× | 0.12× |
| 100,000 | 10.973 | 9.11M | 10.986 | 9.10M | 1.185 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.102 | 0.89× |
| 1 | 5 | 0.371 | 0.508 | 1.37× |
| 1 | 10 | 0.631 | 0.904 | 1.43× |
| 10 | 1 | 0.066 | 0.095 | 1.43× |
| 10 | 5 | 0.324 | 0.430 | 1.33× |
| 10 | 10 | 0.663 | 0.931 | 1.41× |
| 100 | 1 | 0.086 | 0.090 | 1.05× |
| 100 | 5 | 0.309 | 0.428 | 1.38× |
| 100 | 10 | 0.658 | 0.892 | 1.36× |
| 1,000 | 1 | 0.186 | 0.111 | 0.60× |
| 1,000 | 5 | 0.382 | 0.495 | 1.30× |
| 1,000 | 10 | 0.701 | 1.022 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
