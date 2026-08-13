# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.088 | 11.41M | 0.086 | 11.59M | 0.037 | 0.42× | 0.43× |
| 10,000 | 0.744 | 13.44M | 0.949 | 10.53M | 0.088 | 0.12× | 0.09× |
| 100,000 | 7.647 | 13.08M | 7.240 | 13.81M | 0.561 | 0.07× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.132 | 1.24× |
| 1 | 5 | 0.430 | 0.479 | 1.11× |
| 1 | 10 | 0.592 | 0.945 | 1.60× |
| 10 | 1 | 0.068 | 0.095 | 1.39× |
| 10 | 5 | 0.309 | 0.446 | 1.44× |
| 10 | 10 | 0.653 | 0.950 | 1.46× |
| 100 | 1 | 0.075 | 0.101 | 1.35× |
| 100 | 5 | 0.325 | 0.447 | 1.37× |
| 100 | 10 | 0.632 | 0.933 | 1.48× |
| 1,000 | 1 | 0.149 | 0.095 | 0.64× |
| 1,000 | 5 | 0.324 | 0.463 | 1.43× |
| 1,000 | 10 | 0.681 | 1.035 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
