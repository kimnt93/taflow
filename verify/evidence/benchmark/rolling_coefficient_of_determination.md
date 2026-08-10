# RollingCoefficientOfDetermination benchmark (`rolling squared correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.52M | 0.056 | 17.92M | 0.345 | 6.04× | 6.18× |
| 10,000 | 0.546 | 18.30M | 0.543 | 18.42M | 2.265 | 4.15× | 4.17× |
| 100,000 | 5.440 | 18.38M | 5.269 | 18.98M | 25.474 | 4.68× | 4.83× |
| 1,000,000 | 53.614 | 18.65M | 53.367 | 18.74M | 277.586 | 5.18× | 5.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.059 | 0.182 | 3.06× |
| 1 | 5 | 0.386 | 0.799 | 2.07× |
| 1 | 10 | 0.568 | 1.478 | 2.60× |
| 10 | 1 | 0.069 | 0.154 | 2.21× |
| 10 | 5 | 0.239 | 0.750 | 3.14× |
| 10 | 10 | 0.530 | 1.460 | 2.76× |
| 100 | 1 | 0.075 | 0.236 | 3.17× |
| 100 | 5 | 0.250 | 1.358 | 5.43× |
| 100 | 10 | 0.550 | 2.777 | 5.05× |
| 1,000 | 1 | 0.116 | 0.400 | 3.46× |
| 1,000 | 5 | 0.282 | 1.639 | 5.82× |
| 1,000 | 10 | 0.653 | 3.462 | 5.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
