# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 524.45M | 0.001 | 1.12G | 0.029 | 15.31× | 32.72× |
| 10,000 | 0.006 | 1.69G | 0.003 | 3.52G | 0.041 | 6.89× | 14.35× |
| 100,000 | 0.052 | 1.91G | 0.028 | 3.57G | 0.161 | 3.08× | 5.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.154 | 1.29× |
| 1 | 5 | 0.273 | 0.428 | 1.57× |
| 1 | 10 | 0.370 | 0.885 | 2.39× |
| 10 | 1 | 0.043 | 0.079 | 1.87× |
| 10 | 5 | 0.179 | 0.440 | 2.46× |
| 10 | 10 | 0.386 | 0.892 | 2.31× |
| 100 | 1 | 0.042 | 0.080 | 1.90× |
| 100 | 5 | 0.178 | 0.404 | 2.27× |
| 100 | 10 | 0.421 | 0.947 | 2.25× |
| 1,000 | 1 | 0.042 | 0.089 | 2.12× |
| 1,000 | 5 | 0.200 | 0.418 | 2.09× |
| 1,000 | 10 | 0.395 | 0.894 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
