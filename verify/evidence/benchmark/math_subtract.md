# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 421.06M | 0.001 | 969.53M | 0.031 | 13.11× | 30.19× |
| 10,000 | 0.008 | 1.33G | 0.004 | 2.55G | 0.037 | 4.99× | 9.57× |
| 100,000 | 0.063 | 1.60G | 0.037 | 2.70G | 0.073 | 1.17× | 1.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.154 | 2.00× |
| 1 | 5 | 0.278 | 0.450 | 1.62× |
| 1 | 10 | 0.386 | 0.904 | 2.34× |
| 10 | 1 | 0.044 | 0.099 | 2.27× |
| 10 | 5 | 0.289 | 0.449 | 1.55× |
| 10 | 10 | 0.382 | 0.909 | 2.38× |
| 100 | 1 | 0.040 | 0.093 | 2.35× |
| 100 | 5 | 0.174 | 0.408 | 2.34× |
| 100 | 10 | 0.409 | 0.925 | 2.27× |
| 1,000 | 1 | 0.040 | 0.092 | 2.27× |
| 1,000 | 5 | 0.188 | 0.452 | 2.41× |
| 1,000 | 10 | 0.374 | 0.993 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
