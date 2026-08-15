# BreakOfStructureChangeOfCharacter benchmark (`causal BOS and CHOCH events` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.90M | 0.038 | 26.25M | 4.210 | 100.64× | 110.53× |
| 10,000 | 0.431 | 23.18M | 0.408 | 24.51M | 42.380 | 98.25× | 103.85× |
| 100,000 | 4.065 | 24.60M | 4.267 | 23.43M | 441.588 | 108.63× | 103.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.127 | 1.54× |
| 1 | 5 | 0.282 | 0.601 | 2.13× |
| 1 | 10 | 0.458 | 1.003 | 2.19× |
| 10 | 1 | 0.046 | 0.101 | 2.19× |
| 10 | 5 | 0.236 | 0.551 | 2.34× |
| 10 | 10 | 0.462 | 1.023 | 2.21× |
| 100 | 1 | 0.052 | 0.566 | 10.86× |
| 100 | 5 | 0.212 | 2.560 | 12.09× |
| 100 | 10 | 0.459 | 5.144 | 11.20× |
| 1,000 | 1 | 0.096 | 4.537 | 47.24× |
| 1,000 | 5 | 0.246 | 23.503 | 95.42× |
| 1,000 | 10 | 0.695 | 49.159 | 70.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
