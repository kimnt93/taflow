# BetterVolume benchmark (`BetterVolume` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.79M | 0.096 | 10.44M | 0.265 | 2.60× | 2.77× |
| 10,000 | 0.958 | 10.44M | 0.851 | 11.75M | 1.483 | 1.55× | 1.74× |
| 100,000 | 8.350 | 11.98M | 8.181 | 12.22M | 13.654 | 1.64× | 1.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.241 | 2.28× |
| 1 | 5 | 0.416 | 1.080 | 2.60× |
| 1 | 10 | 0.675 | 2.415 | 3.58× |
| 10 | 1 | 0.080 | 0.223 | 2.79× |
| 10 | 5 | 0.330 | 1.052 | 3.18× |
| 10 | 10 | 0.682 | 2.252 | 3.30× |
| 100 | 1 | 0.085 | 0.224 | 2.62× |
| 100 | 5 | 0.333 | 1.307 | 3.93× |
| 100 | 10 | 0.700 | 2.402 | 3.43× |
| 1,000 | 1 | 0.172 | 0.360 | 2.09× |
| 1,000 | 5 | 0.332 | 1.959 | 5.90× |
| 1,000 | 10 | 0.887 | 3.858 | 4.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
