# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.36M | 0.008 | 127.54M | 0.043 | 4.13× | 5.52× |
| 10,000 | 0.070 | 143.06M | 0.066 | 150.83M | 0.121 | 1.73× | 1.82× |
| 100,000 | 0.667 | 149.94M | 0.644 | 155.31M | 0.938 | 1.41× | 1.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.108 | 1.05× |
| 1 | 5 | 0.212 | 0.458 | 2.15× |
| 1 | 10 | 0.440 | 0.964 | 2.19× |
| 10 | 1 | 0.044 | 0.093 | 2.10× |
| 10 | 5 | 0.186 | 0.435 | 2.34× |
| 10 | 10 | 0.409 | 0.990 | 2.42× |
| 100 | 1 | 0.047 | 0.093 | 1.99× |
| 100 | 5 | 0.176 | 0.463 | 2.62× |
| 100 | 10 | 0.407 | 0.995 | 2.44× |
| 1,000 | 1 | 0.046 | 0.099 | 2.18× |
| 1,000 | 5 | 0.210 | 0.551 | 2.63× |
| 1,000 | 10 | 0.446 | 1.062 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
